use std::{
    cell::RefCell,
    rc::Rc
};
use anyhow::{
    bail,
    Result,
};
use crate::{
    impl_construct_key,
    impl_construct_val,
    data::*,
};

#[derive(Clone, Debug)]
pub struct App {
    pub id: usize,
    pub fn_expr: Rc<Expr>,
    pub arg_expr: Option<Rc<Expr>>,
    pub ty: Rc<RefCell<Rc<Ty>>>,
    pub ty_env: Rc<RefCell<TyEnv>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AppKey {
    pub id: usize,
}

impl_construct_val!(App);

impl ConstructVal for App {
    type Key = AppKey;

    fn to_key(&self) -> Self::Key {
        Self::Key {
            id: self.id,
        }
    }
}

impl_construct_key!(AppKey, App, app_store);

impl Construct for AppKey {
    fn logical_name(&self) -> String {
        format!("app.{}", self.id)
    }

    fn description(&self) -> String {
        format!("app[{}]", self.id)
    }
}

impl App {
    pub fn new(ctx: &mut SemantizerContext, fn_expr: Rc<Expr>, arg_expr: Rc<Expr>, ty: Rc<Ty>, ty_env: Rc<RefCell<TyEnv>>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.app_store.next_id(),
            fn_expr,
            arg_expr: Some(arg_expr),
            ty: Rc::new(RefCell::new(ty)),
            ty_env,
        });
        let key = val.to_key();
        ctx.app_store.insert(key.clone(), val.clone()).unwrap()
    }

    pub fn new_as_unary(ctx: &mut SemantizerContext, fn_expr: Rc<Expr>, ty: Rc<Ty>, ty_env: Rc<RefCell<TyEnv>>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.app_store.next_id(),
            fn_expr,
            arg_expr: None,
            ty: Rc::new(RefCell::new(ty)),
            ty_env,
        });
        let key = val.to_key();
        ctx.app_store.insert(key.clone(), val.clone()).unwrap()
    }

    pub fn clone_with_ty_env(self: &Rc<Self>, ctx: &mut SemantizerContext, ty_env: Rc<RefCell<TyEnv>>) -> Result<Rc<Self>> {
        let mut app = self.clone();
        let mut args = Vec::new();
        if app.arg_expr.is_some() {
            loop {
                args.push(app.arg_expr.as_ref().unwrap().clone_with_ty_env(ctx, ty_env.clone()));
                match app.fn_expr.as_ref() {
                    Expr::App(ap) =>
                        app = ap.clone(),
                    Expr::Var(_) =>
                        break,
                    _ => bail!("Non variable function not supported yet: {}", app.fn_expr.description()),
                }
            }
            args.reverse();
        }
        let fn_expr = app.fn_expr.clone_with_ty_env(ctx, ty_env.clone());
        if let Expr::Var(var) = fn_expr.as_ref() {
            if let Some(abs) = var.abs.borrow().as_ref() {
                let abs_ty_env = Rc::new(RefCell::new(abs.ty_env.borrow().clone()));
                let mut fn_ty = abs.ty.borrow().clone();

                let in_tys =
                    args.iter().map(|arg| {
                        let ty = arg.applied_ty(ctx).borrow().clone();
                        ty_env.borrow().apply_env(ctx, ty)
                    }).collect::<Vec<_>>();
                {
                    let mut abs_ty_env_ref = abs_ty_env.borrow_mut();
                    for in_ty in in_tys {
                        fn_ty = abs_ty_env_ref.apply_tys(ctx, fn_ty, in_ty)?;
                    }
                }
                abs.add_child_with_ty_env(ctx, abs_ty_env.clone());
                let ty = fn_expr.applied_ty(ctx).borrow().clone();
                let ty = ty_env.borrow().apply_env(ctx, ty);
                if args.len() == 0 {
                    Ok(Self::new_as_unary(ctx, fn_expr.clone(), ty, abs_ty_env))
                }
                else {
                    let mut ty = ty.to_out_ty();
                    let mut app = Self::new(ctx, fn_expr.clone(), args[0].clone(), ty.clone(), abs_ty_env.clone());
                    for arg in &args[1..] {
                        ty = ty.to_out_ty();
                        app = Self::new(ctx, Expr::new_with_app(app), arg.clone(), ty.clone(), abs_ty_env.clone());
                    }
                    Ok(app)
                }
            }
            else {
                let ty = fn_expr.applied_ty(ctx).borrow().clone();
                let ty = ty_env.borrow().apply_env(ctx, ty);
                if args.len() == 0 {
                    Ok(Self::new_as_unary(ctx, fn_expr.clone(), ty, TyEnv::new_empty()))
                }
                else {
                    let mut ty = ty.to_out_ty();
                    let mut app = Self::new(ctx, fn_expr.clone(), args[0].clone(), ty.clone(), TyEnv::new_empty());
                    for arg in &args[1..] {
                        ty = ty.to_out_ty();
                        app = Self::new(ctx, Expr::new_with_app(app), arg.clone(), ty.clone(), TyEnv::new_empty());
                    }
                    Ok(app)
                }
            }
        }
        else {
            unreachable!();
        }
    }
}

impl AppKey {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}
