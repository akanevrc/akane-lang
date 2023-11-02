use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use inkwell::values::{
    AnyValue,
    AnyValueEnum,
    BasicValue,
    BasicValueEnum,
    FunctionValue,
};
use crate::{
    data::*,
    prelude,
    llvm,
};

pub fn generate(cg_ctx: &mut CodeGenContext, sem_ctx: &mut SemantizerContext) -> Result<()> {
    prelude::init(cg_ctx, sem_ctx)?;
    for (var, abs) in sem_ctx.bind_store.keys_and_vals() {
        generate_fn(cg_ctx, sem_ctx, var.get_val(sem_ctx).unwrap(), abs.clone())?;
    }
    Ok(())
}

fn generate_fn(cg_ctx: &mut CodeGenContext, sem_ctx: &SemantizerContext, var: Rc<Var>, abs: Rc<Abs>) -> Result<()> {
    let i64_ty = TyKey::new_as_base("I64".to_owned()).get_val(sem_ctx).unwrap();
    for arg in &abs.args {
        if arg.ty(sem_ctx).unwrap() != i64_ty {
            bail!("Function arg must be I64 type: {} {}", var.description(), arg.description());
        }
    }
    if abs.expr.ty(sem_ctx).unwrap() != i64_ty {
        bail!("Function must return I64 type: {}", var.description());
    }
    let i64_type = llvm::get_i64_type(cg_ctx)?;
    let function_type = llvm::get_function_type(cg_ctx, &vec![i64_type.into(); abs.args.len()], i64_type.into())?;
    let function = llvm::add_function(cg_ctx, &var.name, function_type)?;
    cg_ctx.bound_values.insert(var.to_key(), function.into()).unwrap();
    cg_ctx.functions.insert(abs.to_key(), function).unwrap();
    let arg_names = abs.args.iter().map(|arg| &arg.name[..]).collect::<Vec<_>>();
    let params = llvm::set_param_names(cg_ctx, function, &arg_names)?;
    for (i, param) in params.into_iter().enumerate() {
        cg_ctx.bound_values.insert(abs.args[i].to_key(), param.into()).unwrap();
    }
    llvm::build_function(cg_ctx, function, |cg_ctx, _| generate_expr(cg_ctx, sem_ctx, abs.expr.clone()))?;
    Ok(())
}

fn generate_expr<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, expr: Rc<Expr>) -> Result<AnyValueEnum<'ctx>> {
    match expr.as_ref() {
        Expr::Var(var) =>
            generate_var(cg_ctx, sem_ctx, var.clone()),
        Expr::Cn(cn) =>
            generate_cn(cg_ctx, sem_ctx, cn.clone()).map(|value| value.into()),
        Expr::Abs(abs) =>
            generate_abs(cg_ctx, sem_ctx, abs.clone()).map(|value| value.as_any_value_enum()),
        Expr::App(app) =>
            generate_app(cg_ctx, sem_ctx, app.clone()).map(|value| value.into()),
    }
}

fn generate_var<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, var: Rc<Var>) -> Result<AnyValueEnum<'ctx>> {
    if let Ok(value) = cg_ctx.bound_values.get(&var.to_key()) {
        if value.is_function_value() {
            if var.ty(sem_ctx).unwrap().rank() == 0 {
                llvm::build_call_without_args(cg_ctx, value.into_function_value())
                .map(|value| value.as_any_value_enum())
            }
            else {
                Ok(value.clone())
            }
        }
        else {
            Ok(value.clone())
        }
    }
    else {
        bail!("Unknown variable: {}", var.description())
    }
}

fn generate_cn<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, cn: Rc<Cn>) -> Result<BasicValueEnum<'ctx>> {
    if let Ok(i64_num) = cn.name.parse::<i64>() {
        llvm::get_const_i64(cg_ctx, i64_num)
        .map(|value| value.as_basic_value_enum())
    }
    else {
        bail!("Not a number: {}", cn.description())
    }
}

fn generate_abs<'ctx>(_cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, _abs: Rc<Abs>) -> Result<FunctionValue<'ctx>> {
    bail!("Not supported yet")
}

fn generate_app<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, app: Rc<App>) -> Result<BasicValueEnum<'ctx>> {
    let mut app = app;
    let mut arguments = Vec::new();
    for rank in 1 .. {
        arguments.push(generate_expr(cg_ctx, sem_ctx, app.arg_expr.clone())?);
        if let Expr::App(fn_app) = app.fn_expr.as_ref() {
            app = fn_app.clone();
        }
        else if rank == app.fn_expr.ty(sem_ctx).unwrap().rank() {
            break;
        }
        else {
            bail!("Number of args does not match: {}", app.fn_expr.description());
        }
    }
    let arguments =
        arguments.into_iter()
        .rev()
        .map(|value| value.into_int_value().into())
        .collect::<Vec<_>>();
    let function = generate_expr(cg_ctx, sem_ctx, app.fn_expr.clone())?.into_function_value();
    llvm::build_call_with_args(cg_ctx, function, &arguments)
}
