use std::{
    cell::RefCell,
    hash::Hash,
    rc::Rc,
};
use crate::{
    impl_id,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Abs {
    pub id: usize,
    pub args: Vec<Rc<Var>>,
    pub expr: Rc<Expr>,
    pub ty: Rc<RefCell<Rc<Ty>>>,
    pub ty_env: Rc<RefCell<TyEnv>>,
    pub children: Rc<RefCell<Vec<Rc<ChildAbs>>>>,
}

#[derive(Clone, Debug)]
pub struct ChildAbs {
    pub args: Vec<Rc<Var>>,
    pub expr: Rc<Expr>,
    pub ty: Rc<RefCell<Rc<Ty>>>,
    pub ty_env: Rc<RefCell<TyEnv>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AbsKey {
    pub id: usize,
}

impl_id!(Abs);

impl Construct for Abs {
    fn logical_name(&self) -> String {
        format!("fn.{}", self.id)
    }

    fn description(&self) -> String {
        format!("fn[{}]", self.id)
    }
}

impl Abs {
    pub fn new_as_var_with_id(ctx: &mut SemantizerContext, id: usize, args: Vec<Rc<Var>>, expr: Rc<Expr>, var: Rc<Var>) -> Rc<Self> {
        let in_tys = args.iter().map(|arg| arg.ty.borrow().clone()).collect();
        let out_ty = expr.ty().borrow().clone();
        let ty = Ty::new_or_get_as_fn_ty(ctx, in_tys, out_ty);
        let tvars = ty.get_tvars();
        let tvars = tvars.into_iter().map(|tvar| tvar.to_key()).collect();
        let ty_env = TyEnv::new(tvars);
        let val = Rc::new(Self {
            id,
            args: args.clone(),
            expr: expr.clone(),
            ty: Rc::new(RefCell::new(ty)),
            ty_env,
            children: Rc::new(RefCell::new(Vec::new())),
        });
        var.abs.replace(Some(val.clone()));
        ctx.abs_store.insert(id, val).unwrap()
    }

    pub fn ret_ty(&self) -> Rc<RefCell<Rc<Ty>>> {
        self.expr.ty()
    }

    pub fn add_child_with_ty_env(&self, ctx: &mut SemantizerContext, ty_env: Rc<RefCell<TyEnv>>) {
        if self.children.borrow().iter().any(|child| child.ty_env == ty_env) {
            return;
        }
        let args = self.args.iter().map(|arg| arg.clone_arg_with_ty_env(ctx, ty_env.clone())).collect();
        let expr = self.expr.clone_with_ty_env(ctx, ty_env.clone());
        let ty = ty_env.borrow().apply_env(ctx, self.ty.borrow().clone());
        let child = Rc::new(ChildAbs {
            args,
            expr,
            ty: Rc::new(RefCell::new(ty)),
            ty_env,
        });
        self.children.borrow_mut().push(child);
    }
}

impl AbsKey {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}
