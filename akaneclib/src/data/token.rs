use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo<'input>(
    pub Token,
    pub StrInfo<'input>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Semicolon,
    Ident(String),
    Num(String),
    OpCode(String),
    Ty,
    Fn,
    Arrow,
    Equal,
    LParen,
    RParen,
}
