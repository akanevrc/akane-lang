use std::rc::Rc;
use crate::{
    impl_construct_val,
    impl_construct_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Base {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BaseKey {
    pub name: String,
}

impl_construct_val!(Base);

impl ConstructVal for Base {
    type Key = BaseKey;

    fn to_key(&self) -> Self::Key {
        Self::Key {
            name: self.name.clone(),
        }
    }
}

impl_construct_key!(BaseKey, Base, base_store);

impl Construct for BaseKey {
    fn logical_name(&self) -> String {
        self.name.logical_name()
    }

    fn description(&self) -> String {
        self.name.description()
    }
}

impl Base {
    pub fn new_or_get(ctx: &mut Context, name: String) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.base_store.next_id(),
            name,
        });
        let key = val.to_key();
        ctx.base_store.insert_or_get(key, val)
    }
}

impl BaseKey {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
