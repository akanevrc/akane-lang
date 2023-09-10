use std::iter::Peekable;
use std::str::Chars;
use anyhow::{
    Error,
    Result,
};
use crate::token::*;

pub fn lex(input: &str) -> Result<Vec<Token>, Vec<Error>> {
    let mut tokens = Vec::new();
    let mut errs = Vec::new();
    let mut chars = input.chars().peekable();
    loop {
        match assume(&mut chars) {
            Ok(Some(Some(token))) =>
                tokens.push(token),
            Ok(Some(None)) =>
                (),
            Ok(None) =>
                break,
            Err(e) => {
                errs.push(e);
                chars.next();
            },
        }
    }
    if errs.len() == 0 {
        Ok(tokens)
    }
    else {
        Err(errs)
    }
}

fn assume(chars: &mut Peekable<Chars>) -> Result<Option<Option<Token>>> {
    if let Some(_) = assume_eof(chars)? {
        Ok(None)
    }
    else if let Some(_) = assume_whitespace(chars)? {
        Ok(Some(None))
    }
    else if let Some(token) = assume_token(chars)? {
        Ok(Some(Some(token)))
    }
    else {
        Err(Error::msg("Invalid token found."))
    }
}

fn assume_eof(chars: &mut Peekable<Chars>) -> Result<Option<()>> {
    if let None = chars.peek() {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_whitespace(chars: &mut Peekable<Chars>) -> Result<Option<()>> {
    let mut consumed = false;
    while is_whitespace(chars.peek()) {
        chars.next();
        consumed = true;
    }
    if consumed {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_token(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    if let Some(token) = assume_semicolon(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_keyword_or_ident(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_num(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_paren(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_symbol_or_op_code(chars)? {
        Ok(Some(token))
    }
    else {
        Ok(None)
    }
}

fn assume_semicolon(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    if is_semicolon(chars.peek()) {
        chars.next();
        Ok(Some(Token::Semicolon))
    }
    else {
        Ok(None)
    }
}

fn assume_keyword_or_ident(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    if is_ident_head(chars.peek()) {
        let mut token = String::new();
        while is_ident_tail(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        if is_fn(&token) {
            Ok(Some(Token::Fn))
        }
        else {
            Ok(Some(Token::Ident(token)))
        }
    }
    else {
        Ok(None)
    }
}

fn assume_num(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    if is_num(chars.peek()) {
        let mut token = String::new();
        while is_num(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        Ok(Some(Token::Num(token)))
    }
    else {
        Ok(None)
    }
}

fn assume_symbol_or_op_code(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    if is_op_code(chars.peek()) {
        let mut token = String::new();
        while is_op_code(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        if is_equal(&token) {
            Ok(Some(Token::Equal))
        }
        else {
            Ok(Some(Token::OpCode(token)))
        }
    }
    else {
        Ok(None)
    }
}

fn assume_paren(chars: &mut Peekable<Chars>) -> Result<Option<Token>> {
    let c = chars.peek();
    if is_l_paren(c) {
        chars.next();
        Ok(Some(Token::LParen))
    }
    else if is_r_paren(c) {
        chars.next();
        Ok(Some(Token::RParen))
    }
    else {
        Ok(None)
    }
}

fn is_whitespace(c: Option<&char>) -> bool {
    c.map_or(false, |c| c.is_ascii_whitespace())
}

fn is_semicolon(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == ';')
}

fn is_ident_head<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '_' || c.is_ascii_alphabetic())
}

fn is_ident_tail<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '_' || c.is_ascii_alphanumeric())
}

fn is_num<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| c.is_ascii_digit())
}

fn is_op_code<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| [
        '!',
        '#',
        '$',
        '%',
        '&',
        '*',
        '+',
        '.',
        '/',
        '<',
        '=',
        '>',
        '?',
        '@',
        '\\',
        '^',
        '|',
        '-',
        '~',
    ].contains(c))
}

fn is_l_paren<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '(')
}

fn is_r_paren<'input>(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == ')')
}

fn is_fn(s: &str) -> bool {
    s == "fn"
}

fn is_equal(s: &str) -> bool {
    s == "="
}

#[cfg(test)]
mod test {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        super::lex(input).unwrap()
    }

    #[test]
    fn test_lex_eof() {
        assert_eq!(lex(""), &[]);
    }

    #[test]
    fn test_lex_whitespace() {
        assert_eq!(lex(" \t\n\r"), &[]);
    }

    #[test]
    fn test_lex_semicolon() {
        assert_eq!(lex(";"), &[Token::Semicolon]);
    }

    #[test]
    fn test_lex_fn() {
        assert_eq!(lex("fn"), &[Token::Fn]);
    }

    #[test]
    fn test_lex_ident() {
        assert_eq!(lex("a"), &[Token::Ident("a".to_string())]);
        assert_eq!(lex("a0"), &[Token::Ident("a0".to_string())]);
        assert_eq!(lex("a_0"), &[Token::Ident("a_0".to_string())]);
        assert_eq!(lex("_a0"), &[Token::Ident("_a0".to_string())]);
        assert_eq!(lex("_a_0"), &[Token::Ident("_a_0".to_string())]);
        assert_eq!(lex("a0_"), &[Token::Ident("a0_".to_string())]);
        assert_eq!(lex("a_0_"), &[Token::Ident("a_0_".to_string())]);
    }

    #[test]
    fn test_lex_num() {
        assert_eq!(lex("0"), &[Token::Num("0".to_string())]);
        assert_eq!(lex("1"), &[Token::Num("1".to_string())]);
        assert_eq!(lex("2"), &[Token::Num("2".to_string())]);
        assert_eq!(lex("3"), &[Token::Num("3".to_string())]);
        assert_eq!(lex("4"), &[Token::Num("4".to_string())]);
        assert_eq!(lex("5"), &[Token::Num("5".to_string())]);
        assert_eq!(lex("6"), &[Token::Num("6".to_string())]);
        assert_eq!(lex("7"), &[Token::Num("7".to_string())]);
        assert_eq!(lex("8"), &[Token::Num("8".to_string())]);
        assert_eq!(lex("9"), &[Token::Num("9".to_string())]);
        assert_eq!(lex("0123456789"), &[Token::Num("0123456789".to_string())]);
    }

    #[test]
    fn test_lex_op_code() {
        assert_eq!(lex("!"), &[Token::OpCode("!".to_string())]);
        assert_eq!(lex("#"), &[Token::OpCode("#".to_string())]);
        assert_eq!(lex("$"), &[Token::OpCode("$".to_string())]);
        assert_eq!(lex("%"), &[Token::OpCode("%".to_string())]);
        assert_eq!(lex("&"), &[Token::OpCode("&".to_string())]);
        assert_eq!(lex("*"), &[Token::OpCode("*".to_string())]);
        assert_eq!(lex("+"), &[Token::OpCode("+".to_string())]);
        assert_eq!(lex("."), &[Token::OpCode(".".to_string())]);
        assert_eq!(lex("/"), &[Token::OpCode("/".to_string())]);
        assert_eq!(lex("<"), &[Token::OpCode("<".to_string())]);
        assert_eq!(lex("="), &[Token::Equal]);
        assert_eq!(lex(">"), &[Token::OpCode(">".to_string())]);
        assert_eq!(lex("?"), &[Token::OpCode("?".to_string())]);
        assert_eq!(lex("@"), &[Token::OpCode("@".to_string())]);
        assert_eq!(lex("\\"), &[Token::OpCode("\\".to_string())]);
        assert_eq!(lex("^"), &[Token::OpCode("^".to_string())]);
        assert_eq!(lex("|"), &[Token::OpCode("|".to_string())]);
        assert_eq!(lex("-"), &[Token::OpCode("-".to_string())]);
        assert_eq!(lex("~"), &[Token::OpCode("~".to_string())]);
    }

    #[test]
    fn test_lex_paren() {
        assert_eq!(lex("("), &[Token::LParen]);
        assert_eq!(lex(")"), &[Token::RParen]);
    }

    #[test]
    fn test_lex_tokens() {
        assert_eq!(lex("fn a = 0;"), &[
            Token::Fn,
            Token::Ident("a".to_string()),
            Token::Equal,
            Token::Num("0".to_string()),
            Token::Semicolon,
        ]);
    }
}
