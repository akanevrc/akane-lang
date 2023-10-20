use std::rc::Rc;
use crate::{
    impl_sem_val,
    impl_sem_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct Ty2Sem {
    pub id: usize,
    pub qual: Rc<QualSem>,
    pub name: String,
    pub in_ty: Rc<TySem>,
    pub out_ty: Rc<TySem>,
    pub arity: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ty2Key {
    pub qual: QualKey,
    pub name: String,
}

impl_sem_val!(
    Ty2Key,
    Ty2Sem,
    Ty2Key {
        qual: self.qual.to_key(),
        name: self.name.clone()
    }
);
impl_sem_key!(Ty2Key, Ty2Sem, ty2_store);

impl Sem for Ty2Key {
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

impl Ty2Sem {
    pub fn new_or_get(ctx: &mut SemContext, qual: Rc<QualSem>, in_ty: Rc<TySem>, out_ty: Rc<TySem>) -> Rc<Self> {
        let arity = out_ty.arity() + 1;
        let val = Rc::new(Self {
            id: ctx.ty2_store.next_id(),
            qual,
            name: Self::name(&in_ty, &out_ty),
            in_ty,
            out_ty,
            arity,
        });
        let key = val.to_key();
        ctx.ty2_store.insert_or_get(key, val)
    }

    fn name(in_ty: &Rc<TySem>, out_ty: &Rc<TySem>) -> String {
        let in_ty_name = match in_ty.as_ref() {
            TySem::Ty2(ty2) => format!("({})", ty2.name),
            TySem::Ty1(ty1) => ty1.name.clone(),
        };
        let out_ty_name = match out_ty.as_ref() {
            TySem::Ty2(ty2) => &ty2.name,
            TySem::Ty1(ty1) => &ty1.name,
        };
        format!("{} -> {}", in_ty_name, out_ty_name)
    }
}

impl Ty2Key {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }
}
