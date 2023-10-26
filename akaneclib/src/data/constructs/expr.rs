use std::{
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};
use crate::data::*;

#[derive(Clone, Debug)]
pub enum Expr {
    Var(Rc<Var>),
    Cn(Rc<Cn>),
    Abs(Rc<Abs>),
    App(Rc<App>),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Var(var), Self::Var(other_var)) =>
                var == other_var,
            (Self::Cn(cn), Self::Cn(other_cn)) =>
                cn == other_cn,
            (Self::Abs(abs), Self::Abs(other_abs)) =>
                abs == other_abs,
            (Self::App(app), Self::App(other_app)) =>
                app == other_app,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Var(var) =>
                (var.id as i128).hash(state),
            Self::Cn(cn) =>
                (cn.id as i128 + (usize::MAX / 2) as i128).hash(state),
            Self::Abs(abs) =>
                (-(abs.id as i128)).hash(state),
            Self::App(app) =>
                (-(app.id as i128 + (usize::MAX / 2) as i128)).hash(state),
        };
    }
}

impl Construct for Expr {
    fn logical_name(&self) -> String {
        match self {
            Self::Var(var) =>
                var.logical_name(),
            Self::Cn(cn) =>
                cn.logical_name(),
            Self::Abs(abs) =>
                abs.logical_name(),
            Self::App(app) =>
                app.logical_name(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Var(var) =>
                var.description(),
            Self::Cn(cn) =>
                cn.description(),
            Self::Abs(abs) =>
                abs.description(),
            Self::App(app) =>
                app.description(),
        }
    }
}

impl Expr {
    pub fn new_as_var(ctx: &mut Context, qual: Rc<Qual>, name: String, ty: Rc<Ty>) -> Rc<Self> {
        Rc::new(Self::Var(Var::new_or_get(ctx, qual, name, ty)))
    }

    pub fn new_as_cn(ctx: &mut Context, name: String, ty: Rc<Ty>) -> Rc<Self> {
        Rc::new(Self::Cn(Cn::new_or_get(ctx, name, ty)))
    }

    pub fn new_as_abs(ctx: &mut Context, args: Vec<Rc<Var>>, expr: Rc<Expr>) -> Rc<Self> {
        Rc::new(Self::Abs(Abs::new(ctx, args, expr)))
    }

    pub fn new_as_app(ctx: &mut Context, fn_expr: Rc<Expr>, arg_expr: Rc<Expr>) -> Rc<Self> {
        Rc::new(Self::App(App::new(ctx, fn_expr, arg_expr)))
    }

    pub fn ty(&self) -> Rc<Ty> {
        match self {
            Self::Var(var) =>
                var.ty.clone(),
            Self::Cn(cn) =>
                cn.ty.clone(),
            Self::Abs(abs) =>
                abs.expr.ty(),
            Self::App(app) =>
                app.fn_expr.ty().to_out_ty(),
        }
    }
}
