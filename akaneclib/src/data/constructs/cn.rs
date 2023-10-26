use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_construct_val,
    impl_construct_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Cn {
    pub id: usize,
    pub name: String,
    pub ty: Rc<Ty>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CnKey {
    pub name: String,
}

impl_construct_val!(Cn);

impl ConstructVal for Cn {
    type Key = CnKey;

    fn to_key(&self) -> Self::Key {
        Self::Key {
            name: self.name.clone(),
        }
    }
}

impl_construct_key!(CnKey, Cn, cn_store);

impl Construct for CnKey {
    fn logical_name(&self) -> String {
        self.name.logical_name()
    }

    fn description(&self) -> String {
        self.name.description()
    }
}

impl Cn {
    pub fn new(ctx: &mut Context, name: String, ty: Rc<Ty>) -> Result<Rc<Self>> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            name,
            ty,
        });
        let key = val.to_key();
        ctx.cn_store.insert(key, val)
    }

    pub fn new_or_get(ctx: &mut Context, name: String, ty: Rc<Ty>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            name,
            ty,
        });
        let key = val.to_key();
        ctx.cn_store.insert_or_get(key, val)
    }

    pub fn get(ctx: &Context, name: String) -> Result<Rc<Self>> {
        let key = CnKey::new(name);
        key.get_val(ctx)
    }
}

impl CnKey {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}