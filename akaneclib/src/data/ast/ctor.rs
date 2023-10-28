use std::rc::Rc;
use crate::data::*;

pub fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
    TopDefEnum::FnDef(fn_def_ast)
}

pub fn fn_def_ast<'input>(ty_annot: Option<Rc<TyAst<'input>>>, left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> FnDefAst<'input> {
    FnDefAst { ty_annot, left_fn_def, expr, str_info }
}

pub fn arrow_ty_ast<'input>(arrow: ArrowAst<'input>, str_info: StrInfo<'input>) -> Rc<TyAst<'input>> {
    Rc::new(TyAst { ty_enum: TyEnum::Arrow(arrow), str_info })
}

pub fn base_ty_ast<'input>(base: BaseAst<'input>, str_info: StrInfo<'input>) -> Rc<TyAst<'input>> {
    Rc::new(TyAst { ty_enum: TyEnum::Base(base), str_info })
}

pub fn arrow_ast<'input>(lhs: Rc<TyAst<'input>>, rhs: Rc<TyAst<'input>>, str_info: StrInfo<'input>) -> ArrowAst<'input> {
    ArrowAst { lhs, rhs, str_info }
}

pub fn base_ast<'input>(name: String, str_info: StrInfo<'input>) -> BaseAst<'input> {
    BaseAst { name, str_info }
}

pub fn left_fn_def_ast<'input>(name: String, args: Vec<String>, str_info: StrInfo<'input>) -> LeftFnDefAst<'input> {
    LeftFnDefAst { name, args, str_info }
}

pub fn app_expr_ast<'input>(app_ast: AppAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::App(app_ast), str_info })
}

pub fn var_expr_ast<'input>(var_ast: VarAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Var(var_ast), str_info })
}

pub fn num_expr_ast<'input>(num_ast: NumAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Num(num_ast), str_info })
}

pub fn app_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> AppAst<'input> {
    AppAst { fn_expr, arg_expr, str_info }
}

pub fn prefix_op_ast<'input>(op_code: String, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>) -> AppAst<'input> {
    app_ast(
        var_expr_ast(var_ast(op_code.clone(), op_code_info.clone()), op_code_info),
        rhs,
        str_info,
    )
}

pub fn infix_op_ast<'input>(op_code: String, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>, lhs_info: StrInfo<'input>) -> AppAst<'input> {
    app_ast(
        app_expr_ast(
            app_ast(
                var_expr_ast(var_ast(op_code, op_code_info.clone()), op_code_info),
                lhs,
                lhs_info.clone(),
            ),
            lhs_info,
        ),
        rhs,
        str_info,
    )
}

pub fn var_ast<'input>(name: String, str_info: StrInfo<'input>) -> VarAst<'input> {
    VarAst { name, str_info }
}

pub fn num_ast<'input>(value: String, str_info: StrInfo<'input>) -> NumAst<'input> {
    NumAst { value, str_info }
}
