use std::rc::Rc;
use anyhow::{
    Error,
    Result,
};
use crate::data::*;
use crate::anyhow_info;

macro_rules! anyhow_ast_with_line {
    ($errs:ident, $ast:expr, $msg:expr, $($arg:tt)*) => {
        {
            let info = &$ast.str_info;
            let target_part_of_line = format!("\n{}", info.target_part_of_line());
            $errs.push(anyhow_info!(info, $msg, $($arg),* target_part_of_line));
            $errs
        }
    };
}

macro_rules! bail_ast_with_line {
    ($errs:ident, $ast:expr, $msg:expr, $($arg:tt)*) => {
        {
            return Err(anyhow_ast_with_line!($errs, $ast, $msg, $($arg),*));
        }
    };
}

macro_rules! visit_with_errors {
    ($result:expr, $errs:ident) => {
        match $result {
            Ok(val) => Ok(val),
            Err(mut es) => {
                $errs.append(&mut es);
                Err(())
            },
        }
    };
}

macro_rules! try_with_errors {
    ($result:expr, $ast:expr, $errs:ident) => {
        match $result {
            Ok(val) => val,
            Err(e) => {
                bail_ast_with_line!($errs, $ast, "{}{}", e);
            },
        }
    };
}

pub fn semantize(ctx: &mut Context, top_def_enums: &[TopDefEnum]) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    for top_def_enum in top_def_enums {
        visit_with_errors!(visit_top_def(ctx, top_def_enum), errs).ok();
    }
    if errs.len() == 0 {
        Ok(())
    }
    else {
        Err(errs)
    }
}

fn visit_top_def(ctx: &mut Context, top_def_enum: &TopDefEnum) -> Result<Rc<Var>, Vec<Error>> {
    Ok(match top_def_enum {
        TopDefEnum::FnDef(fn_def_ast) => visit_fn_def(ctx, fn_def_ast)?,
    })
}

fn visit_fn_def(ctx: &mut Context, fn_def_ast: &FnDefAst) -> Result<Rc<Var>, Vec<Error>> {
    let mut errs = Vec::new();
    let qual = try_with_errors!(ctx.qual_stack.peek().get_val(ctx), fn_def_ast.left_fn_def, errs);
    let name = &fn_def_ast.left_fn_def.name;
    let arg_names = &fn_def_ast.left_fn_def.args;
    let fn_ty =
        if let Some(ty_annot) = &fn_def_ast.ty_annot {
            match visit_with_errors!(visit_ty(ctx, ty_annot), errs) {
                Ok(ty) => ty,
                Err(_) => return Err(errs),
            }
        }
        else {
            let i64_ty = try_with_errors!(TyKey::new_as_base("I64".to_owned()).get_val(ctx), fn_def_ast.left_fn_def, errs);
            let fn_in_tys = vec![i64_ty.clone(); arg_names.len()];
            let fn_out_ty = i64_ty.clone();
            Ty::new_or_get_as_fn_ty(ctx, fn_in_tys, fn_out_ty)
        };
    let var =
        match Var::new(ctx, qual, name.clone()) {
            Ok(var) => var,
            Err(_) =>
                bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Duplicate function definitions: `{}`{}", name),
        };
    var.set_ty(ctx, fn_ty.clone()).unwrap();
    let qual = try_with_errors!(ctx.push_scope_into_qual_stack(Scope::Abs(name.clone())).get_val(ctx), fn_def_ast.left_fn_def, errs);
    let (arg_tys, ret_ty) = fn_ty.to_arg_and_ret_tys();
    if arg_tys.len() != arg_names.len() {
        bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Defferent argument count between type annotation and function definition: `{}`{}", name);
    }
    let args =
        try_with_errors!(
            arg_names.iter()
            .zip(arg_tys)
            .map(|(name, arg_ty)| {
                let var = Var::new(ctx, qual.clone(), name.clone())?;
                var.set_ty(ctx, arg_ty.clone()).unwrap();
                Ok(var)
            })
            .collect::<Result<Vec<_>>>(),
            fn_def_ast.left_fn_def,
            errs
        );
    let expr = visit_expr(ctx, &fn_def_ast.expr)?;
    if !matches!(expr.ty(ctx), Ok(ty) if ty == ret_ty) {
        bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Defferent type between type annotation and function body: `{}`{}", name);
    }
    let abs = Abs::new(ctx, args, expr);
    try_with_errors!(ctx.bind_store.insert(var.to_key(), abs), fn_def_ast.left_fn_def, errs);
    try_with_errors!(ctx.qual_stack.pop(), fn_def_ast.left_fn_def, errs);
    Ok(var)
}

fn visit_ty(ctx: &mut Context, ty_ast: &TyAst) -> Result<Rc<Ty>, Vec<Error>> {
    Ok(match &ty_ast.ty_enum {
        TyEnum::Arrow(arrow) =>
            Rc::new(Ty::Arrow(visit_arrow(ctx, arrow)?)),
        TyEnum::Base(base) =>
            Rc::new(Ty::Base(visit_base(ctx, base)?)),
    })
}

fn visit_arrow(ctx: &mut Context, arrow_ast: &ArrowAst) -> Result<Rc<Arrow>, Vec<Error>> {
    let mut errs = Vec::new();
    match (
        visit_with_errors!(visit_ty(ctx, &arrow_ast.lhs), errs),
        visit_with_errors!(visit_ty(ctx, &arrow_ast.rhs), errs),
    ) {
        (Ok(in_ty), Ok(out_ty)) =>
            Ok(Arrow::new_or_get(ctx, in_ty, out_ty)),
        _ => Err(errs),
    }
}

fn visit_base(ctx: &mut Context, ty_ident_ast: &BaseAst) -> Result<Rc<Base>, Vec<Error>> {
    Ok(Base::new_or_get(ctx, ty_ident_ast.name.clone()))
}

fn visit_expr(ctx: &mut Context, expr_ast: &ExprAst) -> Result<Rc<Expr>, Vec<Error>> {
    Ok(match &expr_ast.expr_enum {
        ExprEnum::App(app_ast) =>
            Rc::new(Expr::App(visit_app(ctx, app_ast)?)),
        ExprEnum::Var(var_ast) =>
            Rc::new(Expr::Var(visit_var(ctx, var_ast)?)),
        ExprEnum::Num(num_ast) =>
            Rc::new(Expr::Cn(visit_num(ctx, num_ast)?)),
    })
}

fn visit_app(ctx: &mut Context, app_ast: &AppAst) -> Result<Rc<App>, Vec<Error>> {
    let mut errs = Vec::new();
    match (
        visit_with_errors!(visit_expr(ctx, &app_ast.fn_expr), errs),
        visit_with_errors!(visit_expr(ctx, &app_ast.arg_expr), errs),
    ) {
        (Ok(fn_expr), Ok(arg_expr)) =>
            Ok(App::new(ctx, fn_expr, arg_expr)),
        _ => Err(errs),
    }
}

fn visit_var(ctx: &mut Context, var_ast: &VarAst) -> Result<Rc<Var>, Vec<Error>> {
    ctx.find_with_qual(|ctx, qual|
        VarKey::new(qual.to_key(), var_ast.name.clone()).get_val(ctx).ok()
    )
    .ok_or_else(|| {
        let mut errs = Vec::new();
        anyhow_ast_with_line!(errs, var_ast, "Unknown variable: `{}`{}", (var_ast.name))
    })
}

fn visit_num(ctx: &mut Context, num_ast: &NumAst) -> Result<Rc<Cn>, Vec<Error>> {
    Ok(Cn::new_or_get(ctx, num_ast.value.clone()))
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use crate::{
        data::*,
        lexer,
        parser,
    };

    fn semantize(s: &str) -> Result<Context, Vec<Error>> {
        let parsed = parser::parse(lexer::lex(s).unwrap()).unwrap();
        let mut ctx = Context::new();
        super::semantize(&mut ctx, &parsed)
        .map(|_| ctx)
    }

    #[test]
    fn test_semantize_id() {
        let mut ctx = semantize("fn id x = x").unwrap();
        let top = Qual::top(&mut ctx).to_key();
        let id = ctx.var_store.get(&VarKey::new(top.clone(), "id".to_owned())).unwrap();
        let i64_ty = TyKey::new_as_base("I64".to_owned()).get_val(&ctx).unwrap();
        assert_eq!(id.name, "id");
        assert_eq!(id.ty(&ctx).unwrap(), Ty::new_or_get_as_fn_ty(&mut ctx, vec![i64_ty.clone()], i64_ty.clone()));
        let x_qual = top.pushed(Scope::Abs("id".to_owned()));
        let x = ctx.var_store.get(&VarKey::new(x_qual, "x".to_owned())).unwrap();
        assert_eq!(x.name, "x");
        assert_eq!(x.ty(&ctx).unwrap(), i64_ty.clone());
        let abs = ctx.bind_store.get(&id.to_key()).unwrap().clone();
        assert_eq!(abs.args.len(), 1);
        assert_eq!(abs.args[0], x);
        assert_eq!(abs.expr.as_ref(), &Expr::Var(x));
        assert_eq!(abs.expr.ty(&ctx).unwrap(), i64_ty);
    }
}
