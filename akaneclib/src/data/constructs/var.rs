use std::{
    cell::RefCell,
    rc::Rc,
};
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
    pub ty: Rc<RefCell<Rc<Ty>>>,
    pub abs: RefCell<Option<Rc<Abs>>>,
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
    pub fn new(ctx: &mut SemantizerContext, qual: Rc<Qual>, name: String, ty: Rc<Ty>) -> Result<Rc<Self>> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            qual,
            name,
            ty: Rc::new(RefCell::new(ty)),
            abs: RefCell::new(None),
        });
        let key = val.to_key();
        ctx.var_store.insert(key, val)
    }

    pub fn new_or_get(ctx: &mut SemantizerContext, qual: Rc<Qual>, name: String, ty: Rc<Ty>) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            qual,
            name,
            ty: Rc::new(RefCell::new(ty)),
            abs: RefCell::new(None),
        });
        let key = val.to_key();
        ctx.var_store.insert_or_get(key, val)
    }

    pub fn is_arg(&self) -> bool {
        self.abs.borrow().is_none()
    }

    pub fn clone_arg_with_ty_env(&self, ctx: &mut SemantizerContext, ty_env: Rc<RefCell<TyEnv>>) -> Rc<Self> {
        let ty = ty_env.borrow().apply_env(ctx, self.ty.borrow().clone());
        Self::new_or_get(ctx, self.qual.clone(), self.name.clone(), ty)
    }
}

impl VarKey {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }
}
