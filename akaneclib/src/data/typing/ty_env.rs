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

#[derive(Clone, Debug, PartialEq)]
pub struct TyEnv {
    tvars: Vec<TVarKey>,
    tys: HashMap<TVarKey, TyKey>,
}

impl TyEnv {
    pub fn new(tvars: Vec<TVarKey>) -> Rc<RefCell<Self>> {
        let unknown = TyKey::unknown();
        let tys = tvars.iter().map(|tvar| (tvar.clone(), unknown.clone())).collect();
        Rc::new(RefCell::new(Self {
            tvars,
            tys,
        }))
    }

    pub fn new_empty() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            tvars: Vec::new(),
            tys: HashMap::new(),
        }))
    }

    pub fn assign(&mut self, tvar: TVarKey, ty: TyKey) -> Result<()> {
        if let Some(mut_ty) = self.tys.get_mut(&tvar) {
            if mut_ty.is_unknown() {
                *mut_ty = ty;
                Ok(())
            } else {
                bail!("Type variable `{}` is already assigned", tvar.description());
            }
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

    pub fn is_unknown(&self) -> bool {
        self.tys.values().any(|ty| ty.is_unknown())
    }

    pub fn is_nondeterministic(&self) -> bool {
        self.tys.values().any(|ty| ty.is_nondeterministic())
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
                if let Some(ty_key) = self.tys.get(&tvar.to_key()) {
                    let applied = ty_key.get_val(ctx).unwrap();
                    if applied.is_unknown() {
                        ty
                    }
                    else {
                        applied
                    }
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

    // pub fn concrete(&self, ctx: &SemantizerContext) -> Result<Vec<Rc<RefCell<TyEnv>>>> {
    //     let mut concrete_tys = Vec::new();
    //     for (tvar, ty) in self.tys.iter() {
    //         concrete_tys.push((tvar.clone(), Self::concrete_ty(ctx, ty.clone())?));
    //     }
    //     let ty_envs =
    //         self.tys_to_ty_envs(&concrete_tys, 0)
    //         .into_iter()
    //         .map(|ty_env| Rc::new(RefCell::new(ty_env)))
    //         .collect();
    //     Ok(ty_envs)
    // }

    // fn concrete_ty(ctx: &SemantizerContext, ty: TyKey) -> Result<Vec<TyKey>> {
    //     match ty {
    //         TyKey::TVar(tvar) => {
    //             let id = tvar.qual.get_val(ctx)?.find_scope(|scope| {
    //                 match scope {
    //                     Scope::Abs(id) => Some(*id)
    //                 }
    //             });
    //             if let Some(id) = id {
    //                 let abs = ctx.abs_store.get(&id).unwrap();
    //                 let ty_env_store = abs.ty_env_store.borrow();
    //                 let tys =
    //                     ty_env_store.iter()
    //                     .map(|ty_env| Self::concrete_ty(ctx, ty_env.borrow().get(&tvar)?))
    //                     .collect::<Result<Vec<_>>>()?
    //                     .into_iter()
    //                     .flat_map(|vec| vec.into_iter())
    //                     .collect();
    //                 Ok(tys)
    //             }
    //             else {
    //                 bail!("Cannot concrete type variable: {}", tvar.description());
    //             }
    //         },
    //         TyKey::Base(_) => Ok(vec![ty]),
    //         TyKey::Arrow(_) => bail!("Arrow type is not supported yet"),
    //     }
    // }

    // fn tys_to_ty_envs(&self, tys: &Vec<(TVarKey, Vec<TyKey>)>, index: usize) -> Vec<TyEnv> {
    //     let mut ty_envs = Vec::new();
    //     if index == tys.len() {
    //         ty_envs.push(self.clone());
    //         return ty_envs;
    //     }
    //     for ty in tys[index].1.iter() {
    //         let mut ty_env = self.clone();
    //         ty_env.tys.insert(tys[index].0.clone(), ty.clone());
    //         ty_envs.extend(ty_env.tys_to_ty_envs(tys, index + 1));
    //     }
    //     ty_envs
    // }
}
