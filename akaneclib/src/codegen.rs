use std::rc::Rc;
use anyhow::{
    anyhow,
    bail,
    Result,
};
use inkwell::{
    values::{
        AnyValueEnum,
        BasicValueEnum,
    },
    module::Linkage,
};
use crate::data::*;

pub fn generate(cg_ctx: &mut CodeGenContext, sem_ctx: &SemantizerContext) -> Result<()> {
    for (var, abs) in sem_ctx.bind_store.keys_and_vals() {
        generate_function(cg_ctx, sem_ctx, var.get_val(sem_ctx).unwrap(), abs.clone())?;
    }
    Ok(())
}

fn generate_function(cg_ctx: &mut CodeGenContext, sem_ctx: &SemantizerContext, var: Rc<Var>, abs: Rc<Abs>) -> Result<()> {
    let i64_ty = TyKey::new_as_base("I64".to_owned()).get_val(sem_ctx).unwrap();
    for arg in &abs.args {
        if arg.ty(sem_ctx).unwrap() != i64_ty {
            bail!("Function arg must be I64 type: {} {}", var.description(), arg.description());
        }
    }
    if abs.expr.ty(sem_ctx).unwrap() != i64_ty {
        bail!("Function must return I64 type: {}", var.description());
    }
    let i64_type = cg_ctx.context.i64_type();
    let function_type = i64_type.fn_type(&vec![i64_type.into(); abs.args.len()], false);
    let function = cg_ctx.module.add_function(&var.name, function_type, Some(Linkage::External));
    cg_ctx.bound_values.insert(var.to_key(), function.into()).unwrap();
    cg_ctx.functions.insert(abs.to_key(), function).unwrap();
    for (i, param) in function.get_param_iter().enumerate() {
        param.set_name(&abs.args[i].name);
        cg_ctx.bound_values.insert(abs.args[i].to_key(), param.into()).unwrap();
    }
    let basic_block = cg_ctx.context.append_basic_block(function, "");
    cg_ctx.builder.position_at_end(basic_block);
    let ret: BasicValueEnum = generate_expr(cg_ctx, sem_ctx, abs.expr.clone())?.try_into().map_err(|_| anyhow!("Not a basic value"))?;
    cg_ctx.builder.build_return(Some(&ret))?;
    Ok(())
}

fn generate_expr<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, expr: Rc<Expr>) -> Result<AnyValueEnum<'ctx>> {
    match expr.as_ref() {
        Expr::Var(var) =>
            generate_var(cg_ctx, sem_ctx, var.clone()),
        Expr::Cn(cn) =>
            generate_cn(cg_ctx, sem_ctx, cn.clone()),
        Expr::Abs(abs) =>
            generate_abs(cg_ctx, sem_ctx, abs.clone()),
        Expr::App(app) =>
            generate_app(cg_ctx, sem_ctx, app.clone()),
    }
}

fn generate_var<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, var: Rc<Var>) -> Result<AnyValueEnum<'ctx>> {
    if let Ok(value) = cg_ctx.bound_values.get(&var.to_key()) {
        if value.is_function_value() {
            if var.ty(sem_ctx).unwrap().rank() == 0 {
                Ok(
                    cg_ctx.builder.build_call(value.into_function_value(), &[], "")?
                    .try_as_basic_value().left().ok_or_else(|| anyhow!("Not a basic value"))?.into()
                )
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

fn generate_cn<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, cn: Rc<Cn>) -> Result<AnyValueEnum<'ctx>> {
    if let Ok(i64_number) = cn.name.parse::<i64>() {
        Ok(cg_ctx.context.i64_type().const_int(i64_number as u64, false).into())
    }
    else {
        bail!("Not a number: {}", cn.description())
    }
}

fn generate_abs<'ctx>(_cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, _abs: Rc<Abs>) -> Result<AnyValueEnum<'ctx>> {
    bail!("Not supported yet")
}

fn generate_app<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, app: Rc<App>) -> Result<AnyValueEnum<'ctx>> {
    let mut app = app;
    let mut arguments = Vec::new();
    for _ in 1 .. app.ty(sem_ctx).unwrap().rank() {
        arguments.push(generate_expr(cg_ctx, sem_ctx, app.arg_expr.clone())?);
        if let Expr::App(fn_app) = app.fn_expr.as_ref() {
            app = fn_app.clone();
        }
        else {
            bail!("Number of args does not match: {}", app.fn_expr.description());
        }
    }
    arguments.push(generate_expr(cg_ctx, sem_ctx, app.arg_expr.clone())?);
    let arguments =
        arguments.into_iter()
        .rev()
        .map(|arg| arg.into_int_value().into())
        .collect::<Vec<_>>();
    let function = generate_expr(cg_ctx, sem_ctx, app.fn_expr.clone())?.into_function_value();
    Ok(
        cg_ctx.builder.build_call(function, &arguments, "")?
        .try_as_basic_value().left().ok_or_else(|| anyhow!("Not a basic value"))?.into()
    )
}
