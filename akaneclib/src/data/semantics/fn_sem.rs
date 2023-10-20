use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_sem_val,
    impl_sem_key,
    data::*,
};

#[derive(Clone, Debug)]
pub struct FnSem {
    pub id: usize,
    pub qual: Rc<QualSem>,
    pub name: String,
    pub ty: Rc<TySem>,
    pub arity: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FnKey {
    pub qual: QualKey,
    pub name: String,
}

impl_sem_val!(
    FnKey,
    FnSem,
    FnKey {
        qual: self.qual.to_key(),
        name: self.name.clone()
    }
);
impl_sem_key!(FnKey, FnSem, fn_store);

impl Sem for FnKey {
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

fn name_with_rank(name: String, rank: usize) -> String {
    format!("{}..{}", name, rank)
}

impl FnSem {
    fn new_one(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>, arity: usize) -> Result<Rc<Self>> {
        let rank = arity - ty.arity();
        let val = Rc::new(Self {
            id: ctx.fn_store.next_id(),
            qual,
            name: name_with_rank(name, rank),
            ty,
            arity,
        });
        let key = val.to_key();
        ctx.fn_store.insert(key, val)
    }

    pub fn new(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>) -> Result<Vec<Rc<Self>>> {
        let mut ty = ty;
        let arity = ty.arity();
        let mut prev_f = Self::new_one(ctx, qual.clone(), name.clone(), ty.clone(), arity)?;
        let mut fs = vec![prev_f.clone()];
        for _ in 1..=arity {
            ty = ty.to_out_ty().unwrap();
            let next_f = Self::new_one(ctx, qual.clone(), name.clone(), ty.clone(), arity)?;
            fs.push(next_f.clone());
            ctx.next_fn_store.insert(prev_f.to_key(), next_f.clone()).unwrap();
            prev_f = next_f;
        }
        ctx.ranked_fn_store.insert(prev_f.to_key().without_rank(), fs.clone()).unwrap();
        Ok(fs)
    }

    fn new_or_get_one(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>, arity: usize) -> Rc<Self> {
        let rank = arity - ty.arity();
        let val = Rc::new(Self {
            id: ctx.fn_store.next_id(),
            qual,
            name: name_with_rank(name, rank),
            ty,
            arity,
        });
        let key = val.to_key();
        ctx.fn_store.insert_or_get(key, val)
    }

    pub fn new_or_get(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>) -> Vec<Rc<Self>> {
        let mut ty = ty;
        let arity = ty.arity();
        let mut prev_f = Self::new_or_get_one(ctx, qual.clone(), name.clone(), ty.clone(), arity);
        let mut fs = vec![prev_f.clone()];
        for _ in 1..=arity {
            ty = ty.to_out_ty().unwrap();
            let next_f = Self::new_or_get_one(ctx, qual.clone(), name.clone(), ty.clone(), arity);
            fs.push(next_f.clone());
            ctx.next_fn_store.insert_or_get(prev_f.to_key(), next_f.clone());
            prev_f = next_f;
        }
        ctx.ranked_fn_store.insert_or_get(prev_f.to_key().without_rank(), fs.clone());
        fs
    }

    pub fn get(ctx: &SemContext, qual: QualKey, name: String) -> Result<Rc<Self>> {
        let key = FnKey::new(qual, name);
        key.get(ctx)
    }

    pub fn get_with_rank(ctx: &SemContext, qual: QualKey, name: String, rank: usize) -> Result<Rc<Self>> {
        let key = FnKey::new_with_rank(qual, name, rank);
        key.get(ctx)
    }

    pub fn rank(&self) -> usize {
        self.arity - self.ty.arity()
    }
}

impl FnKey {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }

    pub fn new_with_rank(qual: QualKey, name: String, rank: usize) -> Self {
        Self { qual, name: name_with_rank(name, rank) }
    }

    pub fn without_rank(&self) -> Self {
        match self.name.find("..") {
            Some(index) => Self { qual: self.qual.clone(), name: self.name[..index].to_owned() },
            None => Self { qual: self.qual.clone(), name: self.name.clone() },
        }
    }

    pub fn get(&self, ctx: &SemContext) -> Result<Rc<FnSem>> {
        ctx.fn_store.get(self)
    }
}
