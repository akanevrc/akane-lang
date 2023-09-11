use std::iter::Peekable;
use anyhow::{
    Error,
    Result,
};
use crate::data::*;

pub fn lex<'input>(input: &'input str) -> Result<Vec<TokenInfo<'input>>, Vec<Error>> {
    let mut tokens = Vec::new();
    let mut errs = Vec::new();
    let mut chars = CharInfos::new(input).peekable();
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

fn assume<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<Option<TokenInfo<'input>>>> {
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

fn assume_eof<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<()>> {
    if let None = chars.peek() {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_whitespace<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<()>> {
    let mut consumed = false;
    while map_char_info(chars.peek(), false, is_whitespace) {
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

fn assume_token<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
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

fn assume_semicolon<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if map_char_info(chars.peek(), false, is_semicolon) {
        let (info, _) = chars.next().unwrap();
        Ok(Some(TokenInfo(Token::Semicolon, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_keyword_or_ident<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if map_char_info(chars.peek(), false, is_ident_head) {
        let (info_head, c) = chars.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while map_char_info(chars.peek(), false, is_ident_tail) {
            let (info, c) = chars.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        if is_fn(&token) {
            Ok(Some(TokenInfo(Token::Fn, info)))
        }
        else {
            Ok(Some(TokenInfo(Token::Ident(token), info)))
        }
    }
    else {
        Ok(None)
    }
}

fn assume_num<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if map_char_info(chars.peek(), false, is_num) {
        let (info_head, c) = chars.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while map_char_info(chars.peek(), false, is_num) {
            let (info, c) = chars.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        Ok(Some(TokenInfo(Token::Num(token), info)))
    }
    else {
        Ok(None)
    }
}

fn assume_symbol_or_op_code<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if map_char_info(chars.peek(), false, is_op_code) {
        let (info_head, c) = chars.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while map_char_info(chars.peek(), false, is_op_code) {
            let (info, c) = chars.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        if is_equal(&token) {
            Ok(Some(TokenInfo(Token::Equal, info)))
        }
        else {
            Ok(Some(TokenInfo(Token::OpCode(token), info)))
        }
    }
    else {
        Ok(None)
    }
}

fn assume_paren<'input>(chars: &mut Peekable<CharInfos<'input>>) -> Result<Option<TokenInfo<'input>>> {
    let c = chars.peek();
    if map_char_info(c, false, is_l_paren) {
        let (info, _) = chars.next().unwrap();
        Ok(Some(TokenInfo(Token::LParen, info)))
    }
    else if map_char_info(c, false, is_r_paren) {
        let (info, _) = chars.next().unwrap();
        Ok(Some(TokenInfo(Token::RParen, info)))
    }
    else {
        Ok(None)
    }
}

fn map_char_info<'input, T>(info: Option<&(StrInfo<'input>, char)>, default: T, f: fn(&char) -> T) -> T {
    match info {
        Some((_, c)) => f(c),
        None => default,
    }
}

fn is_whitespace(c: &char) -> bool {
    c.is_ascii_whitespace()
}

fn is_semicolon(c: &char) -> bool {
    *c == ';'
}

fn is_ident_head(c: &char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_ident_tail(c: &char) -> bool {
    *c == '_' || c.is_ascii_alphanumeric()
}

fn is_num(c: &char) -> bool {
    c.is_ascii_digit()
}

fn is_op_code(c: &char) -> bool {
    [
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
    ].contains(c)
}

fn is_l_paren(c: &char) -> bool {
    *c == '('
}

fn is_r_paren(c: &char) -> bool {
    *c == ')'
}

fn is_fn(s: &str) -> bool {
    s == "fn"
}

fn is_equal(s: &str) -> bool {
    s == "="
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_to_tokens(input: &str) -> Vec<Token> {
        super::lex(input).unwrap().into_iter().map(|TokenInfo(token, _)| token).collect()
    }

    fn lex_to_infos(input: &str) -> Vec<StrInfo> {
        super::lex(input).unwrap().into_iter().map(|TokenInfo(_, info)| info).collect()
    }

    #[test]
    fn test_lex_eof() {
        assert_eq!(lex_to_tokens(""), &[]);
    }

    #[test]
    fn test_lex_whitespace() {
        assert_eq!(lex_to_tokens(" \t\n\r"), &[]);
    }

    #[test]
    fn test_lex_semicolon() {
        assert_eq!(lex_to_tokens(";"), &[Token::Semicolon]);
    }

    #[test]
    fn test_lex_fn() {
        assert_eq!(lex_to_tokens("fn"), &[Token::Fn]);
    }

    #[test]
    fn test_lex_ident() {
        assert_eq!(lex_to_tokens("a"), &[Token::Ident("a".to_string())]);
        assert_eq!(lex_to_tokens("a0"), &[Token::Ident("a0".to_string())]);
        assert_eq!(lex_to_tokens("a_0"), &[Token::Ident("a_0".to_string())]);
        assert_eq!(lex_to_tokens("a0_"), &[Token::Ident("a0_".to_string())]);
    }

    #[test]
    fn test_lex_num() {
        assert_eq!(lex_to_tokens("0"), &[Token::Num("0".to_string())]);
        assert_eq!(lex_to_tokens("1"), &[Token::Num("1".to_string())]);
        assert_eq!(lex_to_tokens("2"), &[Token::Num("2".to_string())]);
        assert_eq!(lex_to_tokens("3"), &[Token::Num("3".to_string())]);
        assert_eq!(lex_to_tokens("4"), &[Token::Num("4".to_string())]);
        assert_eq!(lex_to_tokens("5"), &[Token::Num("5".to_string())]);
        assert_eq!(lex_to_tokens("6"), &[Token::Num("6".to_string())]);
        assert_eq!(lex_to_tokens("7"), &[Token::Num("7".to_string())]);
        assert_eq!(lex_to_tokens("8"), &[Token::Num("8".to_string())]);
        assert_eq!(lex_to_tokens("9"), &[Token::Num("9".to_string())]);
        assert_eq!(lex_to_tokens("0123456789"), &[Token::Num("0123456789".to_string())]);
    }

    #[test]
    fn test_lex_op_code() {
        assert_eq!(lex_to_tokens("!"), &[Token::OpCode("!".to_string())]);
        assert_eq!(lex_to_tokens("#"), &[Token::OpCode("#".to_string())]);
        assert_eq!(lex_to_tokens("$"), &[Token::OpCode("$".to_string())]);
        assert_eq!(lex_to_tokens("%"), &[Token::OpCode("%".to_string())]);
        assert_eq!(lex_to_tokens("&"), &[Token::OpCode("&".to_string())]);
        assert_eq!(lex_to_tokens("*"), &[Token::OpCode("*".to_string())]);
        assert_eq!(lex_to_tokens("+"), &[Token::OpCode("+".to_string())]);
        assert_eq!(lex_to_tokens("."), &[Token::OpCode(".".to_string())]);
        assert_eq!(lex_to_tokens("/"), &[Token::OpCode("/".to_string())]);
        assert_eq!(lex_to_tokens("<"), &[Token::OpCode("<".to_string())]);
        assert_eq!(lex_to_tokens("="), &[Token::Equal]);
        assert_eq!(lex_to_tokens(">"), &[Token::OpCode(">".to_string())]);
        assert_eq!(lex_to_tokens("?"), &[Token::OpCode("?".to_string())]);
        assert_eq!(lex_to_tokens("@"), &[Token::OpCode("@".to_string())]);
        assert_eq!(lex_to_tokens("\\"), &[Token::OpCode("\\".to_string())]);
        assert_eq!(lex_to_tokens("^"), &[Token::OpCode("^".to_string())]);
        assert_eq!(lex_to_tokens("|"), &[Token::OpCode("|".to_string())]);
        assert_eq!(lex_to_tokens("-"), &[Token::OpCode("-".to_string())]);
        assert_eq!(lex_to_tokens("~"), &[Token::OpCode("~".to_string())]);
    }

    #[test]
    fn test_lex_paren() {
        assert_eq!(lex_to_tokens("("), &[Token::LParen]);
        assert_eq!(lex_to_tokens(")"), &[Token::RParen]);
    }

    #[test]
    fn test_lex_tokens() {
        assert_eq!(lex_to_tokens("fn a = 0;"), &[
            Token::Fn,
            Token::Ident("a".to_string()),
            Token::Equal,
            Token::Num("0".to_string()),
            Token::Semicolon,
        ]);
    }

    #[test]
    fn test_lex_returns_infos() {
        let s = "fn a = 0;";
        assert_eq!(lex_to_infos("fn a = 0;"), &[
            StrInfo::new(1, 1, &s[0..2], s),
            StrInfo::new(1, 4, &s[3..4], s),
            StrInfo::new(1, 6, &s[5..6], s),
            StrInfo::new(1, 8, &s[7..8], s),
            StrInfo::new(1, 9, &s[8..9], s),
        ]);
    }
}
