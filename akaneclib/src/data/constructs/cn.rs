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
    pub fn new(ctx: &mut SemContext, name: String) -> Result<Rc<Self>> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            name,
        });
        let key = val.to_key();
        ctx.cn_store.insert(key.clone(), val.clone())?;
        let i64_ty = TyKey::new_as_base("I64".to_owned()).get_val(ctx).unwrap();
        ctx.cn_ty_store.insert(key, i64_ty).unwrap();
        Ok(val)
    }

    pub fn new_or_get(ctx: &mut SemContext, name: String) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.var_store.next_id(),
            name,
        });
        let key = val.to_key();
        match ctx.cn_store.insert(key.clone(), val.clone()) {
            Ok(_) => {
                let i64_ty = TyKey::new_as_base("I64".to_owned()).get_val(ctx).unwrap();
                ctx.cn_ty_store.insert(key, i64_ty).unwrap();
            },
            Err(_) => (),
        }
        val
    }

    pub fn ty(&self, ctx: &SemContext) -> Rc<Ty> {
        ctx.cn_ty_store.get(&self.to_key()).unwrap().clone()
    }
}

impl CnKey {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
