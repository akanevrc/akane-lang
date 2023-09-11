use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo<'input>(
    pub Token,
    pub StrInfo<'input>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Semicolon,
    Ident(String),
    Num(String),
    OpCode(String),
    Fn,
    Equal,
    LParen,
    RParen,
}
