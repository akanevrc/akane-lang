mod qual_stack;

pub use qual_stack::*;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};
use crate::data::*;

pub struct SemantizerContext {
    pub var_store: IdStore<VarKey, Var>,
    pub cn_store: IdStore<CnKey, Cn>,
    pub abs_store: IdStore<AbsKey, Abs>,
    pub app_id: IdVal,
    pub ty_store: IdStore<TyKey, Ty>,
    pub tvar_store: IdStore<TVarKey, TVar>,
    pub base_store: IdStore<BaseKey, Base>,
    pub arrow_store: IdStore<ArrowKey, Arrow>,
    pub qual_store: IdStore<QualKey, Qual>,
    pub qual_stack: QualStack,
    pub var_ty_store: GenericStore<VarKey, Rc<Ty>>,
    pub cn_ty_store: GenericStore<CnKey, Rc<Ty>>,
    pub bind_store: GenericStore<VarKey, Rc<Abs>>,
    pub arg_store: GenericStore<VarKey, (Rc<Abs>, usize)>,
    pub generic_ty_store: GenericStore<TyKey, Rc<RefCell<HashMap<TVarKey, Rc<Ty>>>>>
}

impl SemantizerContext {
    pub fn new() -> Self {
        let mut ctx = Self {
            var_store: IdStore::new(),
            cn_store: IdStore::new(),
            abs_store: IdStore::new(),
            app_id: IdVal::new(),
            ty_store: IdStore::new(),
            tvar_store: IdStore::new(),
            base_store: IdStore::new(),
            arrow_store: IdStore::new(),
            qual_store: IdStore::new(),
            qual_stack: QualStack::new(),
            var_ty_store: GenericStore::new(),
            cn_ty_store: GenericStore::new(),
            bind_store: GenericStore::new(),
            arg_store: GenericStore::new(),
            generic_ty_store: GenericStore::new(),
        };
        let top = Qual::new_or_get(&mut ctx, &QualKey::top());
        Ty::new_or_get_as_base(&mut ctx, "Bottom".to_owned());
        let i64_ty = Ty::new_or_get_as_base(&mut ctx, "I64".to_owned());
        let add = Var::new_or_get(&mut ctx, top.clone(), "add".to_owned());
        let sub = Var::new_or_get(&mut ctx, top.clone(), "sub".to_owned());
        let mul = Var::new_or_get(&mut ctx, top.clone(), "mul".to_owned());
        let div = Var::new_or_get(&mut ctx, top.clone(), "div".to_owned());
        let pipeline_l = Var::new_or_get(&mut ctx, top.clone(), "pipelineL".to_owned());
        let op_ty = Ty::new_or_get_as_fn_ty(&mut ctx, vec![i64_ty.clone(), i64_ty.clone()], i64_ty.clone());
        ctx.var_ty_store.insert(add.to_key(), op_ty.clone()).unwrap();
        ctx.var_ty_store.insert(sub.to_key(), op_ty.clone()).unwrap();
        ctx.var_ty_store.insert(mul.to_key(), op_ty.clone()).unwrap();
        ctx.var_ty_store.insert(div.to_key(), op_ty.clone()).unwrap();
        let pipeline_l_fn_ty = Ty::new_or_get_as_fn_ty(&mut ctx, vec![i64_ty.clone()], i64_ty.clone());
        let pipeline_l_op_ty = Ty::new_or_get_as_fn_ty(&mut ctx, vec![pipeline_l_fn_ty, i64_ty.clone()], i64_ty.clone());
        ctx.var_ty_store.insert(pipeline_l.to_key(), pipeline_l_op_ty.clone()).unwrap();
        Ty::new_or_get_as_base(&mut ctx, "F64".to_owned());
        ctx
    }

    pub fn push_scope_into_qual_stack(&mut self, scope: Scope) -> QualKey {
        let qual_key = self.qual_stack.peek().pushed(scope);
        let qual = Qual::new_or_get(self, &qual_key);
        self.qual_stack.push(&qual)
    }

    pub fn find_with_qual<T>(&self, pred: impl Fn(&Self, Rc<Qual>) -> Option<T>) -> Option<T> {
        for qual in self.qual_stack.iter() {
            match pred(self, qual.get_val(self).unwrap()) {
                Some(x) => return Some(x),
                None => (),
            }
        }
        None
    }
}
