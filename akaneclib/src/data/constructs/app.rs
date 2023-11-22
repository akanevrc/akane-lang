use std::{
    cell::RefCell,
    rc::Rc
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
}

impl AppKey {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}
