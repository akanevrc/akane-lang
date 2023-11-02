use std::rc::Rc;
use anyhow::Result;
use inkwell::values::{
    AnyValueEnum,
    FunctionValue,
};
use crate::{
    data::*,
    llvm,
};

pub fn init<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &mut SemantizerContext) -> Result<()> {
    generate_add(cg_ctx, sem_ctx)?;
    generate_sub(cg_ctx, sem_ctx)?;
    generate_mul(cg_ctx, sem_ctx)?;
    generate_div(cg_ctx, sem_ctx)?;
    Ok(())
}

fn generate_add<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &mut SemantizerContext) -> Result<()> {
    let top = Qual::top(sem_ctx);
    let var = Var::new_or_get(sem_ctx, top, "add".to_owned());
    generate_fn(cg_ctx, var, llvm::build_add)
}

fn generate_sub<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &mut SemantizerContext) -> Result<()> {
    let top = Qual::top(sem_ctx);
    let var = Var::new_or_get(sem_ctx, top, "sub".to_owned());
    generate_fn(cg_ctx, var, llvm::build_sub)
}

fn generate_mul<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &mut SemantizerContext) -> Result<()> {
    let top = Qual::top(sem_ctx);
    let var = Var::new_or_get(sem_ctx, top, "mul".to_owned());
    generate_fn(cg_ctx, var, llvm::build_mul)
}

fn generate_div<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, sem_ctx: &mut SemantizerContext) -> Result<()> {
    let top = Qual::top(sem_ctx);
    let var = Var::new_or_get(sem_ctx, top, "div".to_owned());
    generate_fn(cg_ctx, var, llvm::build_div)
}

fn generate_fn<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, var: Rc<Var>, body: impl FnOnce(&mut CodeGenContext<'ctx>, FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>>) -> Result<()> {
    let i64_type = llvm::get_i64_type(cg_ctx)?;
    let function_type = llvm::get_function_type(cg_ctx, &[i64_type.into(), i64_type.into()], i64_type.into())?;
    let function = llvm::add_function(cg_ctx, &var.name, function_type)?;
    cg_ctx.bound_values.insert(var.to_key(), function.into()).unwrap();
    llvm::build_function(cg_ctx, function, body)?;
    Ok(())
}
