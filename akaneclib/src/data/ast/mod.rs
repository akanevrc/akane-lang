mod ctor;

pub use ctor::*;

use std::rc::Rc;
use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TopDefEnum<'input> {
    FnDef(FnDefAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst<'input> {
    pub ty_annot: Option<Rc<TyAst<'input>>>,
    pub left_fn_def: LeftFnDefAst<'input>,
    pub expr: Rc<ExprAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyAst<'input> {
    pub ty_enum: TyEnum<'input>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyEnum<'input> {
    Arrow(ArrowAst<'input>),
    Base(BaseAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrowAst<'input> {
    pub lhs: Rc<TyAst<'input>>,
    pub rhs: Rc<TyAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BaseAst<'input> {
    pub name: String,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftFnDefAst<'input> {
    pub name: String,
    pub args: Vec<String>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAst<'input> {
    pub expr_enum: ExprEnum<'input>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprEnum<'input> {
    App(AppAst<'input>),
    Var(VarAst<'input>),
    IntNum(IntNumAst<'input>),
    RealNum(RealNumAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppAst<'input> {
    pub fn_expr: Rc<ExprAst<'input>>,
    pub arg_expr: Rc<ExprAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarAst<'input> {
    pub name: String,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntNumAst<'input> {
    pub value: String,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RealNumAst<'input> {
    pub value: String,
    pub str_info: StrInfo<'input>,
}
