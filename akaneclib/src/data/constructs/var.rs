use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_construct_val,
    impl_construct_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Var {
    pub id: usize,
    pub qual: Rc<Qual>,
    pub name: String,
    pub ty: Rc<Ty>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VarKey {
    pub qual: QualKey,
    pub name: String,
}

impl_construct_val!(Var);

impl ConstructVal for Var {
    type Key = VarKey;

    fn to_key(&self) -> Self::Key {
        Self::Key {
            qual: self.qual.to_key(),
            name: self.name.clone(),
        }
    }
}

impl_construct_key!(VarKey, Var, var_store);

impl Construct for VarKey {
    fn logical_name(&self) -> String {
        format!(
            "{}{}",
            self.qual.qualify_logical_name("."),
            self.name.logical_name()
        )
    }

    fn description(&self) -> String {
        format!(
            "{}{}",
            self.qual.qualify_description("."),
            self.name.description()
        )
    }
}

impl Var {
    pub fn new(ctx: &mut Context, qual: Rc<Qual>, name: String, ty: Rc<Ty>) -> Result<Rc<Self>> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            qual,
            name,
            ty,
        });
        let key = val.to_key();
        ctx.var_store.insert(key, val)
    }

    pub fn new_or_get(ctx: &mut Context, qual: Rc<Qual>, name: String, ty: Rc<Ty>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            qual,
            name,
            ty,
        });
        let key = val.to_key();
        ctx.var_store.insert_or_get(key, val)
    }

    pub fn get(ctx: &Context, qual: QualKey, name: String) -> Result<Rc<Self>> {
        let key = VarKey::new(qual, name);
        key.get(ctx)
    }
}

impl VarKey {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }

    pub fn get(&self, ctx: &Context) -> Result<Rc<Var>> {
        ctx.var_store.get(self)
    }
}
