use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_construct_key,
    impl_construct_val,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Abs {
    pub id: usize,
    pub args: Vec<Rc<Var>>,
    pub expr: Rc<Expr>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AbsKey {
    pub id: usize,
}

impl_construct_val!(Abs);

impl ConstructVal for Abs {
    type Key = AbsKey;

    fn to_key(&self) -> Self::Key {
        Self::Key {
            id: self.id,
        }
    }
}

impl_construct_key!(AbsKey, Abs, abs_store);

impl Construct for AbsKey {
    fn logical_name(&self) -> String {
        format!("fn.{}", self.id)
    }

    fn description(&self) -> String {
        format!("fn[{}]", self.id)
    }
}

impl Abs {
    pub fn new(ctx: &mut SemantizerContext, args: Vec<Rc<Var>>, expr: Rc<Expr>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.abs_store.next_id(),
            args,
            expr,
        });
        let key = val.to_key();
        ctx.abs_store.insert_or_get(key, val)
    }

    pub fn ty(&self, ctx: &SemantizerContext) -> Result<Rc<Ty>> {
        self.expr.ty(ctx)
    }
}

impl AbsKey {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}
