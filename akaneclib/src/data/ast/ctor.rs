use std::rc::Rc;
use crate::data::*;

pub fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
    TopDefEnum::Fn(fn_def_ast)
}

pub fn fn_def_ast<'input>(left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> FnDefAst<'input> {
    FnDefAst { left_fn_def, expr, str_info }
}

pub fn left_fn_def_ast<'input>(name: String, args: Vec<String>, str_info: StrInfo<'input>) -> LeftFnDefAst<'input> {
    LeftFnDefAst { name, args, str_info }
}

pub fn fn_expr_ast<'input>(fn_ast: FnAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Fn(fn_ast), str_info })
}

pub fn ident_expr_ast<'input>(ident_ast: IdentAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Ident(ident_ast), str_info })
}

pub fn fn_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> FnAst<'input> {
    FnAst { fn_expr, arg_expr, str_info }
}

pub fn prefix_op_ast<'input>(op_code: String, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>) -> FnAst<'input> {
    fn_ast(
        ident_expr_ast(ident_ast(op_code.clone(), op_code_info.clone()), op_code_info),
        rhs,
        str_info,
    )
}

pub fn infix_op_ast<'input>(op_code: String, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>, lhs_info: StrInfo<'input>) -> FnAst<'input> {
    fn_ast(
        fn_expr_ast(
            fn_ast(
                ident_expr_ast(ident_ast(op_code, op_code_info.clone()), op_code_info),
                lhs,
                lhs_info.clone(),
            ),
            lhs_info,
        ),
        rhs,
        str_info,
    )
}

pub fn ident_ast<'input>(name: String, str_info: StrInfo<'input>) -> IdentAst<'input> {
    IdentAst { name, str_info }
}
