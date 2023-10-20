mod ctor;

pub use ctor::*;

use std::rc::Rc;
use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TopDefEnum<'input> {
    Fn(FnDefAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst<'input> {
    pub ty_annot: Option<Rc<TyExprAst<'input>>>,
    pub left_fn_def: LeftFnDefAst<'input>,
    pub expr: Rc<ExprAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyExprAst<'input> {
    pub expr_enum: TyExprEnum<'input>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyExprEnum<'input> {
    Arrow(TyArrowAst<'input>),
    Ident(TyIdentAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyArrowAst<'input> {
    pub lhs: Rc<TyExprAst<'input>>,
    pub rhs: Rc<TyExprAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyIdentAst<'input> {
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
    Fn(FnAst<'input>),
    Ident(IdentAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst<'input> {
    pub fn_expr: Rc<ExprAst<'input>>,
    pub arg_expr: Rc<ExprAst<'input>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst<'input> {
    pub name: String,
    pub str_info: StrInfo<'input>,
}
