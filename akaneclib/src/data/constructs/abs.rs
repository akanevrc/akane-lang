use std::rc::Rc;
use crate::{
    impl_id,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Abs {
    pub id: usize,
    pub args: Vec<Rc<Var>>,
    pub expr: Rc<Expr>,
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
    pub fn new(ctx: &mut SemContext, args: Vec<Rc<Var>>, expr: Rc<Expr>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.abs_id.next_id(),
            args,
            expr,
        });
        ctx.abs_id.increment();
        val
    }
}
