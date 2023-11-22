use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};
use anyhow::{
    bail,
    Result,
};
use crate::data::*;

#[derive(Debug, PartialEq)]
pub struct TyEnv {
    tys: HashMap<TVarKey, TyKey>
}

impl TyEnv {
    pub fn new(tvars: &Vec<TVarKey>) -> Rc<RefCell<Self>> {
        let bottom = TyKey::bottom();
        let tys = tvars.iter().map(|tvar| (tvar.clone(), bottom.clone())).collect();
        Rc::new(RefCell::new(Self { tys }))
    }

    pub fn new_empty() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { tys: HashMap::new() }))
    }

    pub fn assign(&mut self, tvar: TVarKey, ty: TyKey) -> Result<()> {
        if let Some(mut_ty) = self.tys.get_mut(&tvar) {
            *mut_ty = ty;
            Ok(())
        } else {
            bail!("Unknown type variable: {}", tvar.description());
        }
    }

    pub fn get(&self, tvar: &TVarKey) -> Result<TyKey> {
        if let Some(ty) = self.tys.get(tvar) {
            Ok(ty.clone())
        } else {
            bail!("Unknown type variable: {}", tvar.description());
        }
    }

    pub fn is_generic(&self) -> bool {
        self.tys.len() != 0
    }

    pub fn is_bottom(&self) -> bool {
        self.tys.values().any(|ty| ty.is_bottom())
    }

    pub fn is_nondterministic(&self) -> bool {
        self.tys.values().any(|ty| ty.is_nondterministic())
    }

    pub fn apply_tys(&mut self, ctx: &mut SemantizerContext, applied: Rc<Ty>, applying: Rc<Ty>) -> Result<Rc<Ty>> {
        let tys = applied.apply_from(applying)?;
        let tys = tys.into_iter().map(|(tvar, ty)| (tvar, ty.to_key())).collect::<HashMap<_, _>>();
        self.tys.extend(tys);
        Ok(self.apply_env(ctx, applied.to_out_ty()))
    }

    pub fn apply_env(&self, ctx: &mut SemantizerContext, ty: Rc<Ty>) -> Rc<Ty> {
        match ty.as_ref() {
            Ty::TVar(tvar) =>
                if let Some(ty) = self.tys.get(&tvar.to_key()) {
                    ty.get_val(ctx).unwrap()
                }
                else {
                    ty
                },
            Ty::Base(_) =>
                ty,
            Ty::Arrow(arrow) => {
                let in_ty = self.apply_env(ctx, arrow.in_ty.clone());
                let out_ty = self.apply_env(ctx, arrow.out_ty.clone());
                Ty::new_or_get_as_arrow(ctx, in_ty, out_ty)
            },
        }
    }

    pub fn get_generic_name(&self, default_name: &str) -> String {
        if self.is_generic() {
            let generic_name =
                self.tys.values()
                .map(|ty| ty.logical_name())
                .collect::<Vec<_>>()
                .join(".");
            format!("{}.{}", default_name, generic_name)
        }
        else {
            default_name.to_owned()
        }
    }
}
