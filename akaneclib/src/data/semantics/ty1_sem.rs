use std::rc::Rc;
use crate::{
    impl_sem_val,
    impl_sem_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Ty1Sem {
    pub id: usize,
    pub qual: Rc<QualSem>,
    pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ty1Key {
    pub qual: QualKey,
    pub name: String,
}

impl_sem_val!(
    Ty1Key,
    Ty1Sem,
    Ty1Key {
        qual: self.qual.to_key(),
        name: self.name.clone()
    }
);
impl_sem_key!(Ty1Key, Ty1Sem, ty1_store);

impl Sem for Ty1Key {
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

impl Ty1Sem {
    pub fn new_or_get(ctx: &mut SemContext, qual: Rc<QualSem>, name: String) -> Rc<Self> {
        let val = Rc::new(Self {
            id: ctx.ty1_store.next_id(),
            qual,
            name,
        });
        let key = val.to_key();
        ctx.ty1_store.insert_or_get(key, val)
    }
}

impl Ty1Key {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }
}
