use std::{
    collections::VecDeque,
    rc::Rc,
};
use anyhow::{
    anyhow,
    bail,
    Result,
};
use inkwell::values::{
    AnyValue,
    AnyValueEnum,
    BasicValueEnum,
    FunctionValue,
};
use crate::{
    data::*,
    llvm,
};

pub fn generate(cg_ctx: &mut CodeGenContext, sem_ctx: &mut SemantizerContext) -> Result<()> {
    for (var, abs) in sem_ctx.bind_store.keys_and_vals() {
        generate_fn(cg_ctx, sem_ctx, var.get_val(sem_ctx).unwrap(), abs.clone())?;
    }
    Ok(())
}

fn generate_fn(cg_ctx: &mut CodeGenContext, sem_ctx: &SemantizerContext, var: Rc<Var>, abs: Rc<Abs>) -> Result<()> {
    let arg_tys = abs.args.iter().map(|arg| arg.ty(sem_ctx)).collect::<Result<Vec<_>>>()?;
    let arg_types = arg_tys.iter().map(|ty| llvm::get_type_from_ty(cg_ctx, sem_ctx, ty.clone())).collect::<Result<Vec<_>>>()?;
    let arg_basic_types = arg_types.iter().map(|t| t.clone().try_into().map_err(|_| anyhow!("Not a basic metadata type"))).collect::<Result<Vec<_>>>()?;
    let ret_ty = abs.expr.ty(sem_ctx)?;
    let ret_type = llvm::get_type_from_ty(cg_ctx, sem_ctx, ret_ty)?;
    let ret_basic_type = ret_type.try_into().map_err(|_| anyhow!("Not a basic metadata type"))?;
    let function_type = llvm::get_function_type(cg_ctx, &arg_basic_types, ret_basic_type)?;
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

fn generate_cn<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, cn: Rc<Cn>) -> Result<BasicValueEnum<'ctx>> {
    llvm::get_const_from_cn(cg_ctx, sem_ctx, cn)
}

fn generate_abs<'ctx>(_cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, _abs: Rc<Abs>) -> Result<FunctionValue<'ctx>> {
    bail!("Not supported yet")
}

fn generate_app<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, app: Rc<App>) -> Result<BasicValueEnum<'ctx>> {
    let mut app = app;
    let mut arguments = VecDeque::new();
    for rank in 1 .. {
        arguments.push_front(generate_expr(cg_ctx, sem_ctx, app.arg_expr.clone())?);
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
    if let Expr::Var(var) = app.fn_expr.as_ref() {
        let arguments = arguments.iter().cloned().collect::<Vec<_>>();
        if let Some(value) = generate_special_op(cg_ctx, var.to_key(), &arguments)? {
            return value.try_into().map_err(|_| anyhow!("Not a basic value"));
        }
    }
    let arguments =
        arguments.into_iter()
        .map(|value| value.try_into().map_err(|_| anyhow!("Not a basic metadata value")))
        .collect::<Result<Vec<_>>>()?;
    let function = generate_expr(cg_ctx, sem_ctx, app.fn_expr.clone())?.into_function_value();
    llvm::build_call_with_args(cg_ctx, function, &arguments)
}

fn generate_special_op<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, var_key: VarKey, arguments: &[AnyValueEnum<'ctx>]) -> Result<Option<AnyValueEnum<'ctx>>> {
    if var_key.qual == QualKey::top() {
        Ok(
            match var_key.name.as_str() {
                "add" => Some(llvm::build_add(cg_ctx, arguments[0].into_int_value(), arguments[1].into_int_value())?.as_any_value_enum()),
                "sub" => Some(llvm::build_sub(cg_ctx, arguments[0].into_int_value(), arguments[1].into_int_value())?.as_any_value_enum()),
                "mul" => Some(llvm::build_mul(cg_ctx, arguments[0].into_int_value(), arguments[1].into_int_value())?.as_any_value_enum()),
                "div" => Some(llvm::build_div(cg_ctx, arguments[0].into_int_value(), arguments[1].into_int_value())?.as_any_value_enum()),
                "pipelineL" => {
                    let function = arguments[0].into_function_value();
                    let arguments =
                        arguments[1 ..].iter()
                        .map(|value| value.into_int_value().into())
                        .collect::<Vec<_>>();
                    Some(llvm::build_call_with_args(cg_ctx, function, &arguments)?.into())
                },
                _ => None,
            }
        )
    }
    else {
        Ok(None)
    }
}
