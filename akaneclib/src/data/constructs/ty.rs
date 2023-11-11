use std::{
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};
use crate::{
    impl_construct_key,
    data::*,
};

#[derive(Clone, Debug)]
pub enum Ty {
    TVar(Rc<TVar>),
    Base(Rc<Base>),
    Arrow(Rc<Arrow>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TyKey {
    TVar(TVarKey),
    Base(BaseKey),
    Arrow(ArrowKey),
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TVar(tvar), Self::TVar(other_tvar)) =>
                tvar == other_tvar,
            (Self::Base(base), Self::Base(other_base)) =>
                base == other_base,
            (Self::Arrow(arrow), Self::Arrow(other_arrow)) =>
                arrow == other_arrow,
            _ => false,
        }
    }
}

impl Eq for Ty {}

impl Hash for Ty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::TVar(tvar) =>
                (tvar.id as i128).hash(state),
            Self::Base(base) =>
                (base.id as i128 + (usize::MAX / 2) as i128).hash(state),
            Self::Arrow(arrow) =>
                (-(arrow.id as i128)).hash(state),
        };
    }
}

impl Construct for Ty {
    fn logical_name(&self) -> String {
        self.to_key().logical_name()
    }

    fn description(&self) -> String {
        self.to_key().description()
    }
}

impl ConstructVal for Ty {
    type Key = TyKey;

    fn to_key(&self) -> Self::Key {
        match self {
            Self::TVar(tvar) =>
                Self::Key::TVar(tvar.to_key()),
            Self::Base(base) =>
                Self::Key::Base(base.to_key()),
            Self::Arrow(arrow) =>
                Self::Key::Arrow(arrow.to_key()),
        }
    }
}

impl_construct_key!(TyKey, Ty, ty_store);

impl Construct for TyKey {
    fn logical_name(&self) -> String {
        match self {
            Self::TVar(tvar) =>
                tvar.logical_name(),
            Self::Base(base) =>
                base.logical_name(),
            Self::Arrow(arrow) =>
                arrow.logical_name(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::TVar(tvar) =>
                tvar.description(),
            Self::Base(base) =>
                base.description(),
            Self::Arrow(arrow) =>
                arrow.description(),
        }
    }
}

impl Ty {
    pub fn bottom(ctx: &mut SemantizerContext) -> Rc<Self> {
        TyKey::new_as_base("Bottom".to_owned()).get_val(ctx).unwrap()
    }

    pub fn new_or_get_as_tvar(ctx: &mut SemantizerContext, qual: Rc<Qual>, name: String) -> Rc<Self> {
        let tvar = TVar::new_or_get(ctx, qual, name);
        let val = Rc::new(Self::TVar(tvar.clone()));
        let key = val.to_key();
        if let Ok(_) = ctx.ty_store.insert(key.clone(), val.clone()) {
            let bottom = Self::bottom(ctx);
            ctx.generic_ty_store.insert_into_map(key.clone(), tvar.to_key(), bottom).unwrap();
        }
        val
    }

    pub fn new_or_get_as_base(ctx: &mut SemantizerContext, name: String) -> Rc<Self> {
        let val = Rc::new(Self::Base(Base::new_or_get(ctx, name)));
        let key = val.to_key();
        if let Ok(_) = ctx.ty_store.insert(key.clone(), val.clone()) {
            ctx.generic_ty_store.insert_new_map(key).unwrap();
        }
        val
    }

    pub fn new_or_get_as_arrow(ctx: &mut SemantizerContext, in_ty: Rc<Ty>, out_ty: Rc<Ty>) -> Rc<Self> {
        let val = Rc::new(Self::Arrow(Arrow::new_or_get(ctx, in_ty.clone(), out_ty.clone())));
        let key = val.to_key();
        if let Ok(_) = ctx.ty_store.insert(key.clone(), val.clone()) {
            ctx.generic_ty_store.insert_new_map(key.clone()).unwrap();
            let in_ty_generics = ctx.generic_ty_store.get(&in_ty.to_key()).unwrap();
            let out_ty_generics = ctx.generic_ty_store.get(&out_ty.to_key()).unwrap();
            for (generic_key, generic_val) in in_ty_generics.borrow().iter().chain(out_ty_generics.borrow().iter()) {
                ctx.generic_ty_store.insert_into_map(key.clone(), generic_key.clone(), generic_val.clone()).unwrap();
            }
        }
        val
    }

    pub fn new_or_get_as_fn_ty(ctx: &mut SemantizerContext, in_tys: Vec<Rc<Ty>>, out_ty: Rc<Ty>) -> Rc<Self> {
        let mut ty = out_ty;
        for in_ty in in_tys.into_iter().rev() {
            ty = Self::new_or_get_as_arrow(ctx, in_ty, ty);
        }
        ty
    }

    pub fn to_arg_and_ret_tys(self: Rc<Self>) -> (Vec<Rc<Self>>, Rc<Self>) {
        let mut tys = Vec::new();
        let mut ty = self;
        loop {
            match ty.as_ref() {
                Self::TVar(_) => return (tys, ty.clone()),
                Self::Base(_) => return (tys, ty.clone()),
                Self::Arrow(arrow) => {
                    tys.push(arrow.in_ty.clone());
                    ty = arrow.out_ty.clone();
                },
            }
        }
    }

    pub fn to_out_ty(self: Rc<Self>) -> Rc<Self> {
        match self.as_ref() {
            Self::TVar(_) => self.clone(),
            Self::Base(_) => self.clone(),
            Self::Arrow(arrow) => arrow.out_ty.clone(),
        }
    }

    pub fn rank(&self) -> usize {
        match self {
            Self::TVar(_) => 0,
            Self::Base(_) => 0,
            Self::Arrow(arrow) => arrow.rank,
        }
    }
}

impl TyKey {
    pub fn new_as_tvar(qual: QualKey, name: String) -> Self {
        Self::TVar(TVarKey::new(qual, name))
    }

    pub fn new_as_base(name: String) -> Self {
        Self::Base(BaseKey::new(name))
    }

    pub fn new_as_arrow(in_ty: TyKey, out_ty: TyKey) -> Self {
        Self::Arrow(ArrowKey::new(in_ty, out_ty))
    }
}
