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
        if let Some(ty) = assume_ty(tokens)? {
            ty_annot = Some(ty);
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

fn assume_ty<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>> {
    let mut tys = Vec::new();
    if let Some(lhs) = assume_ty_lhs(tokens)? {
        tys.push(lhs);
        while let Some(rhs) = assume_ty_rhs(tokens)? {
            tys.push(rhs);
        }
        let mut ty_iter = tys.into_iter().rev();
        let mut rhs = ty_iter.next().unwrap();
        for lhs in ty_iter {
            let extended = lhs.str_info.extend(&rhs.str_info);
            rhs = arrow_ty_ast(arrow_ast(lhs, rhs, extended.clone()), extended);
        }
        Ok(Some(rhs))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>> {
    if let Some(term) = assume_ty_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>> {
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

fn assume_ty_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>> {
    if let Some(factor) = assume_ty_factor(tokens)? {
        Ok(Some(factor))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>> {
    if let Some(ty) = assume_ty_paren(tokens)? {
        Ok(Some(ty))
    }
    else if let Some(tvar) = assume_tvar(tokens)? {
        Ok(Some(tvar_ty_ast(tvar.clone(), tvar.str_info.clone())))
    }
    else if let Some(base) = assume_base(tokens)? {
        Ok(Some(base_ty_ast(base.clone(), base.str_info.clone())))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyAst<'input>>>>  {
    if let Some(TokenInfo(Token::LParen, _)) = tokens.peek() {
        tokens.next();
        if let Some(ty) = assume_ty(tokens)? {
            if let Some(TokenInfo(Token::RParen, _)) = tokens.peek() {
                tokens.next();
                return Ok(Some(ty))
            }
            bail_tokens_with_line!(tokens, "`)` required:{}")
        }
        bail_tokens_with_line!(tokens, "Type expression required:{}")
    }
    else {
        Ok(None)
    }
}

fn assume_tvar<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TVarAst<'input>>> {
    if let Some(TokenInfo(Token::LowerIdent(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(tvar_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_base<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<BaseAst<'input>>> {
    if let Some(TokenInfo(Token::UpperIdent(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(base_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_left_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<LeftFnDefAst<'input>>> {
    if let Some(var) = assume_var(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_var(tokens)? {
                args.push(arg);
                continue;
            }
            let extended =
                if let Some(last) = args.last() {
                    var.str_info.extend(&last.str_info)
                }
                else {
                    var.str_info
                };
            let names = args.into_iter().map(|arg| arg.name).collect();
            return Ok(Some(left_fn_def_ast(var.name, names, extended)));
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
            term = app_expr_ast(app_ast(term, f, extended.clone()), extended);
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
            return Ok(Some(app_expr_ast(prefix_op_ast(name, term, extended.clone(), info), extended)));
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
            lhs = app_expr_ast(infix_op_ast(name, lhs.clone(), rhs, extended.clone(), info, lhs.str_info.clone()), extended);
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
    else if let Some(var) = assume_var(tokens)? {
        Ok(Some(var_expr_ast(var.clone(), var.str_info)))
    }
    else if let Some(int_num) = assume_int_num(tokens)? {
        Ok(Some(int_num_expr_ast(int_num.clone(), int_num.str_info)))
    }
    else if let Some(real_num) = assume_real_num(tokens)? {
        Ok(Some(real_num_expr_ast(real_num.clone(), real_num.str_info)))
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

fn assume_var<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<VarAst<'input>>> {
    if let Some(TokenInfo(Token::LowerIdent(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(var_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_int_num<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IntNumAst<'input>>> {
    if let Some(TokenInfo(Token::IntNum(value), info)) = tokens.peek() {
        let value = value.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(int_num_ast(value, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_real_num<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<RealNumAst<'input>>> {
    if let Some(TokenInfo(Token::RealNum(value), info)) = tokens.peek() {
        let value = value.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(real_num_ast(value, info)))
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
    use crate::{
        data::{
            self,
            *,
        },
        lexer,
    };

    fn parse<'input>(s: &'input str) -> Vec<TopDefEnum<'input>> {
        super::parse(lexer::lex(s).unwrap()).unwrap()
    }

    fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
        data::top_fn_def_ast(fn_def_ast)
    }

    fn ty_fn_def_ast<'input>(ty_annot: Rc<TyAst<'input>>, left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>) -> FnDefAst<'input> {
        data::fn_def_ast(Some(ty_annot), left_fn_def, expr, dummy_info())
    }

    fn arrow_ty_ast<'input>(arrow: ArrowAst<'input>) -> Rc<TyAst<'input>> {
        data::arrow_ty_ast(arrow, dummy_info())
    }

    fn tvar_ty_ast<'input>(tvar: TVarAst<'input>) -> Rc<TyAst<'input>> {
        data::tvar_ty_ast(tvar, dummy_info())
    }

    fn base_ty_ast<'input>(base: BaseAst<'input>) -> Rc<TyAst<'input>> {
        data::base_ty_ast(base, dummy_info())
    }

    fn arrow_ast<'input>(lhs: Rc<TyAst<'input>>, rhs: Rc<TyAst<'input>>) -> ArrowAst<'input> {
        data::arrow_ast(lhs, rhs, dummy_info())
    }

    fn tvar_ast<'input>(name: &'input str) -> TVarAst<'input> {
        data::tvar_ast(name.to_owned(), dummy_info())
    }

    fn base_ast<'input>(name: &'input str) -> BaseAst<'input> {
        data::base_ast(name.to_owned(), dummy_info())
    }

    fn fn_def_ast<'input>(left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>) -> FnDefAst<'input> {
        data::fn_def_ast(None, left_fn_def, expr, dummy_info())
    }

    fn left_fn_def_ast<'input>(name: &'input str, args: &[&'input str]) -> LeftFnDefAst<'input> {
        data::left_fn_def_ast(name.to_owned(), args.to_owned().into_iter().map(|s| s.to_owned()).collect(), dummy_info())
    }

    fn app_expr_ast<'input>(app_ast: AppAst<'input>) -> Rc<ExprAst<'input>> {
        data::app_expr_ast(app_ast, dummy_info())
    }

    fn var_expr_ast<'input>(var_ast: VarAst<'input>) -> Rc<ExprAst<'input>> {
        data::var_expr_ast(var_ast, dummy_info())
    }

    fn int_num_expr_ast<'input>(int_num_ast: IntNumAst<'input>) -> Rc<ExprAst<'input>> {
        data::int_num_expr_ast(int_num_ast, dummy_info())
    }

    fn real_num_expr_ast<'input>(real_num_ast: RealNumAst<'input>) -> Rc<ExprAst<'input>> {
        data::real_num_expr_ast(real_num_ast, dummy_info())
    }

    fn app_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>) -> AppAst<'input> {
        data::app_ast(fn_expr, arg_expr, dummy_info())
    }

    fn prefix_op_ast<'input>(op_code: &'input str, rhs: Rc<ExprAst<'input>>) -> AppAst<'input> {
        data::prefix_op_ast(op_code.to_owned(), rhs, dummy_info(), dummy_info())
    }

    fn infix_op_ast<'input>(op_code: &'input str, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>) -> AppAst<'input> {
        data::infix_op_ast(op_code.to_owned(), lhs, rhs, dummy_info(), dummy_info(), dummy_info())
    }

    fn var_ast<'input>(name: &'input str) -> VarAst<'input> {
        data::var_ast(name.to_owned(), dummy_info())
    }

    fn int_num_ast<'input>(value: &'input str) -> IntNumAst<'input> {
        data::int_num_ast(value.to_owned(), dummy_info())
    }

    fn real_num_ast<'input>(value: &'input str) -> RealNumAst<'input> {
        data::real_num_ast(value.to_owned(), dummy_info())
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
                    int_num_expr_ast(int_num_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f a b = 0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b"]),
                    int_num_expr_ast(int_num_ast("0")),
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
                    var_expr_ast(var_ast("a")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = f"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    var_expr_ast(var_ast("f")),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_int_num() {
        assert_eq!(parse("fn f = 0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    int_num_expr_ast(int_num_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = 123"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    int_num_expr_ast(int_num_ast("123")),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_real_num() {
        assert_eq!(parse("fn f = 0.0"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    real_num_expr_ast(real_num_ast("0.0")),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = 125.125"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    real_num_expr_ast(real_num_ast("125.125")),
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
                    app_expr_ast(
                        app_ast(
                            var_expr_ast(var_ast("g")),
                            var_expr_ast(var_ast("a")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f g a b = g a b"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["g", "a", "b"]),
                    app_expr_ast(
                        app_ast(
                            app_expr_ast(
                                app_ast(
                                    var_expr_ast(var_ast("g")),
                                    var_expr_ast(var_ast("a")),
                                ),
                            ),
                            var_expr_ast(var_ast("b")),
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
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            var_expr_ast(var_ast("a")),
                            int_num_expr_ast(int_num_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f g a b c = g a + g b + c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["g", "a", "b", "c"]),
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            app_expr_ast(
                                infix_op_ast(
                                    "add",
                                    app_expr_ast(
                                        app_ast(
                                            var_expr_ast(var_ast("g")),
                                            var_expr_ast(var_ast("a")),
                                        ),
                                    ),
                                    app_expr_ast(
                                        app_ast(
                                            var_expr_ast(var_ast("g")),
                                            var_expr_ast(var_ast("b")),
                                        ),
                                    ),
                                ),
                            ),
                            var_expr_ast(var_ast("c")),
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
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            app_expr_ast(
                                infix_op_ast(
                                    "mul",
                                    var_expr_ast(var_ast("a")),
                                    var_expr_ast(var_ast("b")),
                                ),
                            ),
                            var_expr_ast(var_ast("c")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f a b c = a + b * c"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &["a", "b", "c"]),
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            var_expr_ast(var_ast("a")),
                            app_expr_ast(
                                infix_op_ast(
                                    "mul",
                                    var_expr_ast(var_ast("b")),
                                    var_expr_ast(var_ast("c")),
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
                    app_expr_ast(
                        infix_op_ast(
                            "pipelineL",
                            var_expr_ast(var_ast("a")),
                            app_expr_ast(
                                infix_op_ast(
                                    "pipelineL",
                                    var_expr_ast(var_ast("b")),
                                    var_expr_ast(var_ast("c")),
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
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            app_expr_ast(
                                infix_op_ast(
                                    "add",
                                    var_expr_ast(var_ast("a")),
                                    var_expr_ast(var_ast("b")),
                                ),
                            ),
                            var_expr_ast(var_ast("c")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = a + (b + c)"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            var_expr_ast(var_ast("a")),
                            app_expr_ast(
                                infix_op_ast(
                                    "add",
                                    var_expr_ast(var_ast("b")),
                                    var_expr_ast(var_ast("c")),
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
                    app_expr_ast(
                        prefix_op_ast(
                            "negate",
                            int_num_expr_ast(int_num_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
        assert_eq!(parse("fn f = -a + 1"), &[
            top_fn_def_ast(
                fn_def_ast(
                    left_fn_def_ast("f", &[]),
                    app_expr_ast(
                        infix_op_ast(
                            "add",
                            app_expr_ast(
                                prefix_op_ast(
                                    "negate",
                                    var_expr_ast(var_ast("a")),
                                ),
                            ),
                            int_num_expr_ast(int_num_ast("1")),
                        ),
                    ),
                ),
            ),
        ]);
    }

    #[test]
    fn test_parse_ty_annot() {
        assert_eq!(parse("ty I64 -> I64 fn f a = 0"), &[
            top_fn_def_ast(
                ty_fn_def_ast(
                    arrow_ty_ast(
                        arrow_ast(
                            base_ty_ast(base_ast("I64")),
                            base_ty_ast(base_ast("I64")),
                        ),
                    ),
                    left_fn_def_ast("f", &["a"]),
                    int_num_expr_ast(int_num_ast("0")),
                ),
            ),
        ]);
        assert_eq!(parse("ty (I64 -> a) -> I64 -> a fn f a b = a b"), &[
            top_fn_def_ast(
                ty_fn_def_ast(
                    arrow_ty_ast(
                        arrow_ast(
                            arrow_ty_ast(
                                arrow_ast(
                                    base_ty_ast(base_ast("I64")),
                                    tvar_ty_ast(tvar_ast("a")),
                                ),
                            ),
                            arrow_ty_ast(
                                arrow_ast(
                                    base_ty_ast(base_ast("I64")),
                                    tvar_ty_ast(tvar_ast("a")),
                                ),
                            ),
                        ),
                    ),
                    left_fn_def_ast("f", &["a", "b"]),
                    app_expr_ast(
                        app_ast(
                            var_expr_ast(var_ast("a")),
                            var_expr_ast(var_ast("b")),
                        ),
                    ),
                ),
            ),
        ]);
    }
}
