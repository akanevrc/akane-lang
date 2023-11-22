use std::rc::Rc;
use crate::data::*;

pub fn init_prelude(ctx: &mut SemantizerContext) {
    init_quals(ctx);
    init_tys(ctx);
    init_vars(ctx);
}

fn init_quals(ctx: &mut SemantizerContext) {
    Qual::new_or_get(ctx, &QualKey::top());
}

fn init_tys(ctx: &mut SemantizerContext) {
    Ty::new_or_get_as_base(ctx, "Bottom".to_owned());
    Ty::new_or_get_as_base(ctx, "I64".to_owned());
    Ty::new_or_get_as_base(ctx, "F64".to_owned());
}

fn init_vars(ctx: &mut SemantizerContext) {
    init_op(ctx, "add".to_owned());
    init_op(ctx, "sub".to_owned());
    init_op(ctx, "mul".to_owned());
    init_op(ctx, "div".to_owned());
}

fn init_op(ctx: &mut SemantizerContext, name: String) {
    let top = ctx.qual_stack.peek().get_val(ctx).unwrap();
    let id = ctx.abs_id.next_id();
    ctx.abs_id.increment();
    let qual = ctx.push_scope_into_qual_stack(Scope::Abs(id)).get_val(ctx).unwrap();
    let a_ty = Ty::new_or_get_as_tvar(ctx, qual.clone(), "a".to_owned());
    let a_fn_ty = Ty::new_or_get_as_fn_ty(ctx, vec![a_ty.clone()], a_ty.clone());
    let var_ty = Ty::new_or_get_as_fn_ty(ctx, vec![a_ty.clone(), a_ty.clone()], a_ty.clone());
    let var = Var::new_or_get(ctx, top.clone(), name, var_ty.clone());
    let (arg_tys, _) = var_ty.clone().to_arg_and_ret_tys();
    let x = Var::new_or_get(ctx, qual.clone(), "x".to_owned(), arg_tys[0].clone());
    let y = Var::new_or_get(ctx, qual.clone(), "y".to_owned(), arg_tys[1].clone());
    let inner_var_name = format!("_{}", var.name);
    let inner_var = Var::new_or_get(ctx, top.clone(), inner_var_name, var_ty.clone());
    let tvar =
        if let TyKey::TVar(tvar) = a_ty.to_key() {
            tvar
        }
        else {
            unreachable!()
        };
    let ty_env = TyEnv::new(&vec![tvar]);
    let expr = Rc::new(Expr::App(App::new(ctx, Rc::new(Expr::Var(inner_var.clone())), Rc::new(Expr::Var(x.clone())), a_fn_ty, ty_env.clone())));
    let expr = Rc::new(Expr::App(App::new(ctx, expr, Rc::new(Expr::Var(y.clone())), a_ty, ty_env)));
    Abs::new_as_var_with_id(ctx, id, vec![x, y], expr, var);
    ctx.qual_stack.pop().unwrap();
}
