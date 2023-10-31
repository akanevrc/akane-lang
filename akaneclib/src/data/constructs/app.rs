use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_id,
    data::*,
};

#[derive(Clone, Debug)]
pub struct App {
    pub id: usize,
    pub fn_expr: Rc<Expr>,
    pub arg_expr: Rc<Expr>,
}

impl_id!(App);

impl Construct for App {
    fn logical_name(&self) -> String {
        format!("app.{}", self.id)
    }

    fn description(&self) -> String {
        format!("app[{}]", self.id)
    }
}

impl App {
    pub fn new(ctx: &mut SemantizerContext, fn_expr: Rc<Expr>, arg_expr: Rc<Expr>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.app_id.next_id(),
            fn_expr,
            arg_expr,
        });
        ctx.app_id.increment();
        val
    }

    pub fn ty(&self, ctx: &SemantizerContext) -> Result<Rc<Ty>> {
        self.fn_expr.ty(ctx).map(|ty| ty.to_out_ty())
    }
}
