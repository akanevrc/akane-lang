mod prelude;

use std::rc::Rc;
use crate::data::*;

use self::prelude::init_prelude;

#[derive(Debug)]
pub struct SemantizerContext {
    pub var_store: IdStore<VarKey, Var>,
    pub cn_store: IdStore<CnKey, Cn>,
    pub abs_store: GenericStore<usize, Rc<Abs>>,
    pub abs_id: IdVal,
    pub app_store: IdStore<AppKey, App>,
    pub ty_store: IdStore<TyKey, Ty>,
    pub tvar_store: IdStore<TVarKey, TVar>,
    pub base_store: IdStore<BaseKey, Base>,
    pub arrow_store: IdStore<ArrowKey, Arrow>,
    pub qual_store: IdStore<QualKey, Qual>,
    pub qual_stack: QualStack,
}

impl SemantizerContext {
    pub fn new() -> Self {
        let mut ctx = Self {
            var_store: IdStore::new(),
            cn_store: IdStore::new(),
            abs_store: GenericStore::new(),
            abs_id: IdVal::new(),
            app_store: IdStore::new(),
            ty_store: IdStore::new(),
            tvar_store: IdStore::new(),
            base_store: IdStore::new(),
            arrow_store: IdStore::new(),
            qual_store: IdStore::new(),
            qual_stack: QualStack::new(),
        };
        init_prelude(&mut ctx);
        ctx
    }

    pub fn push_scope_into_qual_stack(&mut self, scope: Scope) -> QualKey {
        let qual_key = self.qual_stack.peek().pushed(scope);
        let qual = Qual::new_or_get(self, &qual_key);
        self.qual_stack.push(&qual)
    }

    pub fn find_with_qual<T>(&self, pred: impl Fn(&Self, Rc<Qual>) -> Option<T>) -> Option<T> {
        for qual in self.qual_stack.iter() {
            if let Some(t) = pred(self, qual.get_val(self).unwrap()) {
                return Some(t);
            }
        }
        None
    }
}
