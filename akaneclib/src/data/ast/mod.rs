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
    pub left_fn_def: LeftFnDefAst<'input>,
    pub expr: Rc<ExprAst<'input>>,
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
