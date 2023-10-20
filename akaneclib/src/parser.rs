use std::{
    iter::Peekable,
    rc::Rc,
};
use anyhow::{
    Error,
    Result,
};
use crate::data::*;
use crate::bail_info;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Assoc {
    L,
    R,
}

macro_rules! bail_tokens_with_line {
    ($tokens:expr, $msg:literal) => {
        {
            let info = &$tokens.peek().unwrap().1;
            let target_part_of_line = format!("\n{}", info.target_part_of_line());
            bail_info!(info, $msg, target_part_of_line);
        }
    };
}

pub fn parse<'input>(input: Vec<TokenInfo<'input>>) -> Result<Vec<TopDefEnum<'input>>, Vec<Error>> {
    let mut asts = Vec::new();
    let mut errs = Vec::new();
    let mut tokens = input.into_iter().peekable();
    loop {
        if tokens.peek().is_none() {
            break;
        }
        match assume(&mut tokens) {
            Ok(Some(ast)) =>
                asts.push(ast.clone()),
            Ok(None) =>
                break,
            Err(e) => {
                errs.push(e);
                tokens.next();
            },
        }
    }
    if errs.len() == 0 {
        Ok(asts)
    }
    else {
        Err(errs)
    }
}

fn assume<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum<'input>>> {
    if let Some(_) = assume_eof(tokens)? {
        Ok(None)
    }
    else if let Some(ast) = assume_top_def(tokens)? {
        Ok(Some(ast))
    }
    else {
        bail_tokens_with_line!(tokens, "Invalid top-level definition:{}");
    }
}

fn assume_eof<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<()>> {
    if let Some(_) = assume_simple_token(tokens, Token::Eof)? {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_top_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum<'input>>> {
    if let Some(ast) = assume_fn_def(tokens)? {
        if let Some(_) = assume_simple_token(tokens, Token::Eof)? {
            Ok(Some(top_fn_def_ast(ast)))
        }
        else if let Some(_) = assume_simple_token(tokens, Token::Semicolon)? {
            Ok(Some(top_fn_def_ast(ast)))
        }
        else {
            bail_tokens_with_line!(tokens, "`;` required:{}");
        }
    }
    else {
        Ok(None)
    }
}

fn assume_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<FnDefAst<'input>>> {
    let mut ty_annot = None;
    let mut ty_info = None;
    if let Some(info) = assume_simple_token(tokens, Token::Ty)? {
        ty_info = Some(info);
        if let Some(ty_expr) = assume_ty_expr(tokens)? {
            ty_annot = Some(ty_expr);
        }
    }
    if let Some(fn_info) = assume_simple_token(tokens, Token::Fn)? {
        if let Some(left_fn_def) = assume_left_fn_def(tokens)? {
            if let Some(_) = assume_simple_token(tokens, Token::Equal)? {
                if let Some(expr) = assume_expr(tokens)? {
                    let extended =
                        if let Some(ty_info) = ty_info {
                            ty_info.extend(&expr.str_info)
                        }
                        else {
                            fn_info.extend(&expr.str_info)
                        };
                    return Ok(Some(fn_def_ast(ty_annot, left_fn_def, expr, extended)));
                }
                bail_tokens_with_line!(tokens, "Expression required:{}");
            }
            bail_tokens_with_line!(tokens, "`=` required:{}");
        }
        bail_tokens_with_line!(tokens, "Left function definition required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_ty_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    let mut exprs = Vec::new();
    if let Some(lhs) = assume_ty_lhs(tokens)? {
        exprs.push(lhs);
        while let Some(rhs) = assume_ty_rhs(tokens)? {
            exprs.push(rhs);
        }
        let mut expr_iter = exprs.into_iter().rev();
        let mut rhs = expr_iter.next().unwrap();
        for lhs in expr_iter {
            let extended = lhs.str_info.extend(&rhs.str_info);
            rhs = ty_arrow_expr_ast(ty_arrow_ast(lhs, rhs, extended.clone()), extended);
        }
        Ok(Some(rhs))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(term) = assume_ty_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(TokenInfo(Token::Arrow, _)) = tokens.peek() {
        tokens.next();
        if let Some(term) = assume_ty_term(tokens)? {
            return Ok(Some(term));
        }
        bail_tokens_with_line!(tokens, "Type term required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_ty_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(factor) = assume_ty_factor(tokens)? {
        Ok(Some(factor))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(expr) = assume_ty_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ty_ident(tokens)? {
        Ok(Some(ty_ident_expr_ast(ident.clone(), ident.str_info.clone())))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>>  {
    if let Some(TokenInfo(Token::LParen, _)) = tokens.peek() {
        tokens.next();
        if let Some(expr) = assume_ty_expr(tokens)? {
            if let Some(TokenInfo(Token::RParen, _)) = tokens.peek() {
                tokens.next();
                return Ok(Some(expr))
            }
            bail_tokens_with_line!(tokens, "`)` required:{}")
        }
        bail_tokens_with_line!(tokens, "Type expression required:{}")
    }
    else {
        Ok(None)
    }
}

fn assume_ty_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TyIdentAst<'input>>> {
    if let Some(TokenInfo(Token::Ident(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ty_ident_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_left_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<LeftFnDefAst<'input>>> {
    if let Some(ident) = assume_ident(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_ident(tokens)? {
                args.push(arg);
                continue;
            }
            let extended =
                if let Some(last) = args.last() {
                    ident.str_info.extend(&last.str_info)
                }
                else {
                    ident.str_info
                };
            let names = args.into_iter().map(|arg| arg.name).collect();
            return Ok(Some(left_fn_def_ast(ident.name, names, extended)));
        }
    }
    else {
        Ok(None)
    }
}

fn assume_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(lhs) = assume_prefix_op_lhs(tokens)? {
        let expr = assume_infix_op_rhs(tokens, 0, lhs)?;
        Ok(Some(expr))
    }
    else {
        Ok(None)
    }
}

fn assume_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(factor) = assume_factor(tokens)? {
        let mut term = factor.clone();
        while let Some(f) = assume_factor(tokens)? {
            let extended = factor.str_info.extend(&f.str_info);
            term = fn_expr_ast(fn_ast(term, f, extended.clone()), extended);
        }
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_prefix_op_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(TokenInfo(Token::OpCode(op_code), info)) = tokens.peek() {
        let op_code = op_code.clone();
        let info = info.clone();
        tokens.next();
        if let Some(term) = assume_term(tokens)? {
            let name = prefix_op_name(&op_code, tokens)?;
            let extended = info.extend(&term.str_info);
            return Ok(Some(fn_expr_ast(prefix_op_ast(name, term, extended.clone(), info), extended)));
        }
        bail_tokens_with_line!(tokens, "Term required:{}");
    }
    else if let Some(term) = assume_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_infix_op_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>, expr_prec: usize, mut lhs: Rc<ExprAst<'input>>) -> Result<Rc<ExprAst<'input>>> {
    while let Some(TokenInfo(Token::OpCode(op_code), info)) = tokens.peek() {
        let op_code = op_code.clone();
        let info = info.clone();
        let (prec, assoc) = op_code_precedence(&op_code, tokens)?;
        if prec < expr_prec {
            return Ok(lhs);
        }
        tokens.next();
        if let Some(rhs) = assume_term(tokens)? {
            let mut rhs = rhs.clone();
            if let Some(TokenInfo(Token::OpCode(next_op_code), _)) = tokens.peek() {
                let next_op_code = next_op_code.clone();
                let (next_prec, _) = op_code_precedence(&next_op_code, tokens)?;
                if assoc == Assoc::L && prec < next_prec {
                    rhs = assume_infix_op_rhs(tokens, prec + 1, rhs)?;
                }
                else if assoc == Assoc::R && prec <= next_prec {
                    rhs = assume_infix_op_rhs(tokens, prec, rhs)?;
                }
            }
            let name = infix_op_name(&op_code, tokens)?;
            let extended = lhs.str_info.extend(&rhs.str_info);
            lhs = fn_expr_ast(infix_op_ast(name, lhs.clone(), rhs, extended.clone(), info, lhs.str_info.clone()), extended);
        }
        else {
            bail_tokens_with_line!(tokens, "Term required:{}");
        }
    }
    Ok(lhs)
}

fn assume_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(expr) = assume_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ident(tokens)? {
        Ok(Some(ident_expr_ast(ident.clone(), ident.str_info)))
    }
    else if let Some(num) = assume_num(tokens)? {
        Ok(Some(ident_expr_ast(num.clone(), num.str_info)))
    }
    else {
        Ok(None)
    }
}

fn assume_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>>  {
    if let Some(TokenInfo(Token::LParen, _)) = tokens.peek() {
        tokens.next();
        if let Some(expr) = assume_expr(tokens)? {
            if let Some(TokenInfo(Token::RParen, _)) = tokens.peek() {
                tokens.next();
                return Ok(Some(expr))
            }
            bail_tokens_with_line!(tokens, "`)` required:{}")
        }
        bail_tokens_with_line!(tokens, "Expression required:{}")
    }
    else {
        Ok(None)
    }
}

fn assume_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst<'input>>> {
    if let Some(TokenInfo(Token::Ident(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ident_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_num<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst<'input>>> {
    if let Some(TokenInfo(Token::Num(value), info)) = tokens.peek() {
        let value = value.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ident_ast(value, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_simple_token<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>, assumed: Token) -> Result<Option<StrInfo<'input>>> {
    if let Some(TokenInfo(token, info)) = tokens.peek() {
        if *token == assumed {
            let info = info.clone();
            tokens.next();
            Ok(Some(info))
        }
        else {
            Ok(None)
        }
    }
    else {
        Ok(None)
    }
}

fn prefix_op_name<'input>(op_code: &str, tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<String> {
    match op_code {
        "-" => Ok("negate".to_string()),
        _ => bail_tokens_with_line!(tokens, "Invalid prefix operator:{}"),
    }
}

fn infix_op_name<'input>(op_code: &str, tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<String> {
    match op_code {
        "+" => Ok("add".to_string()),
        "-" => Ok("sub".to_string()),
        "*" => Ok("mul".to_string()),
        "/" => Ok("div".to_string()),
        "<|" => Ok("pipelineL".to_string()),
        _ => bail_tokens_with_line!(tokens, "Invalid infix operator:{}"),
    }
}

fn op_code_precedence<'input>(op_code: &str, tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<(usize, Assoc)> {
    match op_code {
        "*" | "/" => Ok((7, Assoc::L)),
        "+" | "-" => Ok((6, Assoc::L)),
        "<|" => Ok((1, Assoc::R)),
        _ => bail_tokens_with_line!(tokens, "Invalid infix operator:{}"),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::data::{
        self,
        *,
    };

    fn parse<'input>(s: &'input str) -> Vec<TopDefEnum<'input>> {
        super::parse(crate::lexer::lex(s).unwrap()).unwrap()
    }

    fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
        data::top_fn_def_ast(fn_def_ast)
    }

    fn ty_fn_def_ast<'input>(ty_annot: Rc<TyExprAst<'input>>, left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>) -> FnDefAst<'input> {
        data::fn_def_ast(Some(ty_annot), left_fn_def, expr, dummy_info())
    }

    fn ty_arrow_expr_ast<'input>(ty_arrow: TyArrowAst<'input>) -> Rc<TyExprAst<'input>> {
        data::ty_arrow_expr_ast(ty_arrow, dummy_info())
    }

    fn ty_ident_expr_ast<'input>(ty_ident: TyIdentAst<'input>) -> Rc<TyExprAst<'input>> {
        data::ty_ident_expr_ast(ty_ident, dummy_info())
    }

    fn ty_arrow_ast<'input>(lhs: Rc<TyExprAst<'input>>, rhs: Rc<TyExprAst<'input>>) -> TyArrowAst<'input> {
        data::ty_arrow_ast(lhs, rhs, dummy_info())
    }

    fn ty_ident_ast<'input>(name: &'input str) -> TyIdentAst<'input> {
        data::ty_ident_ast(name.to_owned(), dummy_info())
    }

    fn fn_def_ast<'input>(left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>) -> FnDefAst<'input> {
        data::fn_def_ast(None, left_fn_def, expr, dummy_info())
    }

    fn left_fn_def_ast<'input>(name: &'input str, args: &[&'input str]) -> LeftFnDefAst<'input> {
        data::left_fn_def_ast(name.to_owned(), args.to_owned().into_iter().map(|s| s.to_owned()).collect(), dummy_info())
    }

    fn fn_expr_ast<'input>(fn_ast: FnAst<'input>) -> Rc<ExprAst<'input>> {
        data::fn_expr_ast(fn_ast, dummy_info())
    }

    fn ident_expr_ast<'input>(ident_ast: IdentAst<'input>) -> Rc<ExprAst<'input>> {
        data::ident_expr_ast(ident_ast, dummy_info())
    }

    fn fn_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>) -> FnAst<'input> {
        data::fn_ast(fn_expr, arg_expr, dummy_info())
    }

    fn prefix_op_ast<'input>(op_code: &'input str, rhs: Rc<ExprAst<'input>>) -> FnAst<'input> {
        data::prefix_op_ast(op_code.to_owned(), rhs, dummy_info(), dummy_info())
    }

    fn infix_op_ast<'input>(op_code: &'input str, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>) -> FnAst<'input> {
        data::infix_op_ast(op_code.to_owned(), lhs, rhs, dummy_info(), dummy_info(), dummy_info())
    }

    fn ident_ast<'input>(name: &'input str) -> IdentAst<'input> {
        data::ident_ast(name.to_owned(), dummy_info())
    }

    fn dummy_info<'a>() -> StrInfo<'a> {
        StrInfo::eof()
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse(""), &[]);
    }

    #[test]
    fn test_parse_arg() {
        assert_eq!(parse("fn f a = 0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a"]),
                    ident_expr_ast(ident_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f a b = 0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b"]),
                    ident_expr_ast(ident_ast("0")),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_ident() {
        assert_eq!(parse("fn f a = a"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a"]),
                    ident_expr_ast(ident_ast("a")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = f"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    ident_expr_ast(ident_ast("f")),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_num() {
        assert_eq!(parse("fn f = 0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    ident_expr_ast(ident_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = 123"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    ident_expr_ast(ident_ast("123")),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_fn() {
        assert_eq!(parse("fn f g a = g a"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["g", "a"]),
                    fn_expr_ast(
                        fn_ast(
                            ident_expr_ast(ident_ast("g")),
                            ident_expr_ast(ident_ast("a")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f g a b = g a b"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["g", "a", "b"]),
                    fn_expr_ast(
                        fn_ast(
                            fn_expr_ast(
                                fn_ast(
                                    ident_expr_ast(ident_ast("g")),
                                    ident_expr_ast(ident_ast("a")),
                                ),
                            ),
                            ident_expr_ast(ident_ast("b")),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_infix_op() {
        assert_eq!(parse("fn f a = a + 1"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            ident_expr_ast(ident_ast("a")),
                            ident_expr_ast(ident_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f g a b c = g a + g b + c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["g", "a", "b", "c"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            fn_expr_ast(
                                infix_op_ast(
                                    "add",
                                    fn_expr_ast(
                                        fn_ast(
                                            ident_expr_ast(ident_ast("g")),
                                            ident_expr_ast(ident_ast("a")),
                                        ),
                                    ),
                                    fn_expr_ast(
                                        fn_ast(
                                            ident_expr_ast(ident_ast("g")),
                                            ident_expr_ast(ident_ast("b")),
                                        ),
                                    ),
                                ),
                            ),
                            ident_expr_ast(ident_ast("c")),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_infix_op_prec() {
        assert_eq!(parse("fn f a b c = a * b + c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b", "c"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            fn_expr_ast(
                                infix_op_ast(
                                    "mul",
                                    ident_expr_ast(ident_ast("a")),
                                    ident_expr_ast(ident_ast("b")),
                                ),
                            ),
                            ident_expr_ast(ident_ast("c")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f a b c = a + b * c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b", "c"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            ident_expr_ast(ident_ast("a")),
                            fn_expr_ast(
                                infix_op_ast(
                                    "mul",
                                    ident_expr_ast(ident_ast("b")),
                                    ident_expr_ast(ident_ast("c")),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_infix_op_right_assoc() {
        assert_eq!(parse("fn f a b c = a <| b <| c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b", "c"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "pipelineL",
                            ident_expr_ast(ident_ast("a")),
                            fn_expr_ast(
                                infix_op_ast(
                                    "pipelineL",
                                    ident_expr_ast(ident_ast("b")),
                                    ident_expr_ast(ident_ast("c")),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_paren() {
        assert_eq!(parse("fn f a b c = (a + b) + c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b", "c"]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            fn_expr_ast(
                                infix_op_ast(
                                    "add",
                                    ident_expr_ast(ident_ast("a")),
                                    ident_expr_ast(ident_ast("b")),
                                ),
                            ),
                            ident_expr_ast(ident_ast("c")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = a + (b + c)"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            ident_expr_ast(ident_ast("a")),
                            fn_expr_ast(
                                infix_op_ast(
                                    "add",
                                    ident_expr_ast(ident_ast("b")),
                                    ident_expr_ast(ident_ast("c")),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_prefix_op() {
        assert_eq!(parse("fn f = -1"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    fn_expr_ast(
                        prefix_op_ast(
                            "negate",
                            ident_expr_ast(ident_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = -a + 1"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    fn_expr_ast(
                        infix_op_ast(
                            "add",
                            fn_expr_ast(
                                prefix_op_ast(
                                    "negate",
                                    ident_expr_ast(ident_ast("a")),
                                ),
                            ),
                            ident_expr_ast(ident_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_ty_annot() {
        assert_eq!(parse("ty i64 -> i64 fn f a = 0"), &[
            top_fn_def_ast(
                ty_fn_def_ast(
                    ty_arrow_expr_ast(
                        ty_arrow_ast(
                            ty_ident_expr_ast(ty_ident_ast("i64")),
                            ty_ident_expr_ast(ty_ident_ast("i64")),
                        ),
                    ),
                    left_fn_def_ast("f", &["a"]),
                    ident_expr_ast(ident_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("ty (i64 -> i64) -> i64 -> i64 fn f a b = a b"), &[
            top_fn_def_ast(
                ty_fn_def_ast(
                    ty_arrow_expr_ast(
                        ty_arrow_ast(
                            ty_arrow_expr_ast(
                                ty_arrow_ast(
                                    ty_ident_expr_ast(ty_ident_ast("i64")),
                                    ty_ident_expr_ast(ty_ident_ast("i64")),
                                ),
                            ),
                            ty_arrow_expr_ast(
                                ty_arrow_ast(
                                    ty_ident_expr_ast(ty_ident_ast("i64")),
                                    ty_ident_expr_ast(ty_ident_ast("i64")),
                                ),
                            ),
                        ),
                    ),
                    left_fn_def_ast("f", &["a", "b"]),
                    fn_expr_ast(
                        fn_ast(
                            ident_expr_ast(ident_ast("a")),
                            ident_expr_ast(ident_ast("b")),
                        ),
                    ),
                ),
            ),
        ]);
    }
}
