use std::rc::Rc;
use crate::{
    impl_id,
    data::*,
};

#[derive(Clone, Debug)]
pub struct App {
    pub id: usize,
    pub f: Rc<Expr>,
    pub arg: Rc<Expr>,
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
    pub fn new(ctx: &mut Context, f: Rc<Expr>, arg: Rc<Expr>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.app_id.next_id(),
            f,
            arg,
        });
        ctx.app_id.increment();
        val
    }
}
