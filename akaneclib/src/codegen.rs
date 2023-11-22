use std::{
    cell::RefCell,
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
    let var_abs =
        sem_ctx.var_store.vals()
        .filter(|var| var.abs.borrow().is_some())
        .map(|var| (var.clone(), var.abs.borrow().clone().unwrap()))
        .collect::<Vec<_>>();
    for (_, abs) in var_abs.iter() {
        let mut ty_env_store = abs.ty_env_store.borrow_mut();
        ty_env_store.unify();
    }
    for (var, abs) in var_abs {
        let ty_env_store = abs.ty_env_store.borrow();
        if ty_env_store.is_generic() {
            for ty_env in ty_env_store.iter() {
                generate_fn(cg_ctx, sem_ctx, var.clone(), abs.clone(), ty_env.clone())?;
            }
        }
        else {
            let ty_env = TyEnv::new_empty();
            generate_fn(cg_ctx, sem_ctx, var, abs.clone(), ty_env)?;
        }
    }
    Ok(())
}

fn generate_fn(cg_ctx: &mut CodeGenContext, sem_ctx: &mut SemantizerContext, var: Rc<Var>, abs: Rc<Abs>, ty_env: Rc<RefCell<TyEnv>>) -> Result<()> {
    let ty_env_ref = ty_env.borrow();
    if ty_env_ref.is_bottom() {
        return Ok(());
    }
    else if ty_env_ref.is_nondterministic() {
        bail!("Nondeterministic types is not supported yet");
    }
    let arg_tys =
        abs.args.iter()
        .map(|arg| ty_env_ref.apply_env(sem_ctx, arg.ty.borrow().clone()))
        .collect::<Vec<_>>();
    let arg_types = arg_tys.iter().map(|ty| llvm::get_type_from_ty(cg_ctx, sem_ctx, ty.clone())).collect::<Result<Vec<_>>>()?;
    let arg_basic_types = arg_types.iter().map(|t| t.clone().try_into().map_err(|_| anyhow!("Not a basic metadata type"))).collect::<Result<Vec<_>>>()?;
    let ret_ty = ty_env_ref.apply_env(sem_ctx, abs.expr.ty().borrow().clone());
    let ret_type = llvm::get_type_from_ty(cg_ctx, sem_ctx, ret_ty)?;
    let ret_basic_type = ret_type.try_into().map_err(|_| anyhow!("Not a basic metadata type"))?;
    let function_type = llvm::get_function_type(cg_ctx, &arg_basic_types, ret_basic_type)?;
    let function_name = ty_env_ref.get_generic_name(&var.logical_name());
    eprintln!("{}", function_name);
    let function = llvm::add_function(cg_ctx, &function_name, function_type)?;
    cg_ctx.bound_values.insert(function_name.clone(), function.into()).unwrap();
    cg_ctx.functions.insert(function_name, function).unwrap();
    let arg_names = abs.args.iter().map(|arg| &arg.name[..]).collect::<Vec<_>>();
    let params = llvm::set_param_names(cg_ctx, function, &arg_names)?;
    for (i, param) in params.into_iter().enumerate() {
        let argument_name = ty_env_ref.get_generic_name(&abs.args[i].logical_name());
        cg_ctx.bound_values.insert(argument_name, param.into()).unwrap();
    }
    cg_ctx.ty_env_stack.push(ty_env.clone());
    llvm::build_function(cg_ctx, function, |cg_ctx, _| generate_expr(cg_ctx, sem_ctx, abs.expr.clone(), TyEnv::new_empty()))?;
    cg_ctx.ty_env_stack.pop();
    Ok(())
}

fn generate_expr<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, expr: Rc<Expr>, ty_env: Rc<RefCell<TyEnv>>) -> Result<AnyValueEnum<'ctx>> {
    match expr.as_ref() {
        Expr::Var(var) =>
            generate_var(cg_ctx, sem_ctx, var.clone(), ty_env),
        Expr::Cn(cn) =>
            generate_cn(cg_ctx, sem_ctx, cn.clone()).map(|value| value.into()),
        Expr::Abs(abs) =>
            generate_abs(cg_ctx, sem_ctx, abs.clone()).map(|value| value.as_any_value_enum()),
        Expr::App(app) =>
            generate_app(cg_ctx, sem_ctx, app.clone()).map(|value| value.into()),
    }
}

fn generate_var<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, _sem_ctx: &SemantizerContext, var: Rc<Var>, ty_env: Rc<RefCell<TyEnv>>) -> Result<AnyValueEnum<'ctx>> {
    let ty_env =
        if var.is_arg() {
            cg_ctx.ty_env_stack.last().unwrap().clone()
        }
        else {
            ty_env
        };
    let function_name = ty_env.borrow().get_generic_name(&var.logical_name());
    if let Ok(value) = cg_ctx.bound_values.get(&function_name) {
        Ok(value.clone())
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
    let mut arguments = Vec::new();
    if app.arg_expr.is_some() {
        for rank in 1 .. {
            arguments.push(generate_expr(cg_ctx, sem_ctx, app.arg_expr.as_ref().unwrap().clone(), TyEnv::new_empty())?);
            if let Expr::App(fn_app) = app.fn_expr.as_ref() {
                app = fn_app.clone();
            }
            else if rank == app.fn_expr.ty().borrow().rank() {
                break;
            }
            else {
                bail!("Number of args does not match: {}", app.fn_expr.description());
            }
        }
    }
    if let Expr::Var(var) = app.fn_expr.as_ref() {
        arguments.reverse();
        if let Some(value) = generate_special_op(cg_ctx, var.to_key(), &arguments)? {
            return value.try_into().map_err(|_| anyhow!("Not a basic value"));
        }
        else if var.is_arg() {
            let value = generate_expr(cg_ctx, sem_ctx, app.fn_expr.clone(), TyEnv::new_empty())?;
            return value.try_into().map_err(|_| anyhow!("Not a basic value"));
        }
        let arguments =
            arguments.into_iter()
            .map(|value| value.try_into().map_err(|_| anyhow!("Not a basic metadata value")))
            .collect::<Result<Vec<_>>>()?;
        let function = generate_expr(cg_ctx, sem_ctx, app.fn_expr.clone(), app.ty_env.clone())?.into_function_value();
        llvm::build_call(cg_ctx, function, &arguments)
    }
    else {
        bail!("Non variable function not supported yet: {}", app.fn_expr.description());
    }
}

fn generate_special_op<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, var_key: VarKey, arguments: &[AnyValueEnum<'ctx>]) -> Result<Option<AnyValueEnum<'ctx>>> {
    if var_key.qual == QualKey::top() {
        Ok(
            match var_key.name.as_str() {
                "_add" => Some(llvm::build_add(cg_ctx, arguments[0], arguments[1])?),
                "_sub" => Some(llvm::build_sub(cg_ctx, arguments[0], arguments[1])?),
                "_mul" => Some(llvm::build_mul(cg_ctx, arguments[0], arguments[1])?),
                "_div" => Some(llvm::build_div(cg_ctx, arguments[0], arguments[1])?),
                "_pipelineL" => {
                    let function = arguments[0].into_function_value();
                    let arguments =
                        arguments[1 ..].iter()
                        .map(|value| value.into_int_value().into())
                        .collect::<Vec<_>>();
                    Some(llvm::build_call(cg_ctx, function, &arguments)?.into())
                },
                _ => None,
            }
        )
    }
    else {
        Ok(None)
    }
}
