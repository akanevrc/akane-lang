mod qual_stack;
mod id_store;
mod generic_store;

pub use qual_stack::*;
pub use id_store::*;
pub use generic_store::*;

use std::rc::Rc;
use crate::data::*;

pub struct SemContext {
    pub qual_stack: QualStack,
    pub qual_store: IdStore<QualKey, QualSem>,
    pub ty_store: IdStore<TyKey, TySem>,
    pub ty2_store: IdStore<Ty2Key, Ty2Sem>,
    pub ty1_store: IdStore<Ty1Key, Ty1Sem>,
    pub fn_store: IdStore<FnKey, FnSem>,
    pub ranked_fn_store: GenericStore<FnKey, Vec<Rc<FnSem>>>,
    pub next_fn_store: GenericStore<FnKey, Rc<FnSem>>,
}

impl SemContext {
    pub fn new() -> Self {
        let mut ctx = Self {
            qual_stack: QualStack::new(),
            qual_store: IdStore::new(),
            ty_store: IdStore::new(),
            ty2_store: IdStore::new(),
            ty1_store: IdStore::new(),
            fn_store: IdStore::new(),
            ranked_fn_store: GenericStore::new(),
            next_fn_store: GenericStore::new(),
        };
        let top = QualSem::top(&mut ctx);
        let i64_ty = TySem::new_or_get_ty1(&mut ctx, top.clone(), "i64".to_owned());
        let i64_uni_op_ty = TySem::new_or_get_fn_ty(&mut ctx, top.clone(), vec![i64_ty.clone()], i64_ty.clone());
        FnSem::new_or_get(&mut ctx, top.clone(), "negate".to_owned(), i64_uni_op_ty);
        let i64_bin_op_ty = TySem::new_or_get_fn_ty(&mut ctx, top.clone(), vec![i64_ty.clone(), i64_ty.clone()], i64_ty.clone());
        FnSem::new_or_get(&mut ctx, top.clone(), "add".to_owned(), i64_bin_op_ty.clone());
        FnSem::new_or_get(&mut ctx, top.clone(), "sub".to_owned(), i64_bin_op_ty.clone());
        FnSem::new_or_get(&mut ctx, top.clone(), "mul".to_owned(), i64_bin_op_ty.clone());
        FnSem::new_or_get(&mut ctx, top.clone(), "div".to_owned(), i64_bin_op_ty.clone());
        let pipeline_l_ty = TySem::new_or_get_fn_ty(&mut ctx, top.clone(), vec![i64_bin_op_ty, i64_ty.clone()], i64_ty);
        FnSem::new_or_get(&mut ctx, top, "pipelineL".to_owned(), pipeline_l_ty);
        ctx
    }

    pub fn push_scope_into_qual_stack(&mut self, scope: ScopeSem) -> QualKey {
        let qual_key = self.qual_stack.peek().pushed(scope);
        let qual = QualSem::new_or_get(self, &qual_key);
        self.qual_stack.push(&qual)
    }

    pub fn find_with_qual<T>(&self, f: impl Fn(&Self, Rc<QualSem>) -> Option<T>) -> Option<T> {
        for qual in self.qual_stack.iter() {
            match f(self, qual.get_val(self).unwrap()) {
                Some(x) => return Some(x),
                None => (),
            }
        }
        None
    }
}
