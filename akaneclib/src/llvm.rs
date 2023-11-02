use anyhow::{
    anyhow,
    Result,
};
use inkwell::{
    module::Linkage,
    types::{
        BasicTypeEnum,
        BasicMetadataTypeEnum,
        FunctionType,
        IntType,
    },
    values::{
        AnyValueEnum,
        BasicValueEnum,
        BasicMetadataValueEnum,
        FunctionValue,
        IntValue,
    },
};
use crate::data::*;

pub fn get_const_i64<'ctx>(cg_ctx: &CodeGenContext<'ctx>, value: i64) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.context.i64_type().const_int(value as u64, false))
}

pub fn build_call_without_args<'ctx>(cg_ctx: &CodeGenContext<'ctx>, function: FunctionValue<'ctx>) -> Result<BasicValueEnum<'ctx>> {
    cg_ctx.builder.build_call(function, &[], "")?
    .try_as_basic_value().left().ok_or_else(|| anyhow!("Not a basic value"))
}

pub fn build_call_with_args<'ctx>(cg_ctx: &CodeGenContext<'ctx>, function: FunctionValue<'ctx>, arguments: &[BasicMetadataValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>> {
    cg_ctx.builder.build_call(function, arguments, "")?
    .try_as_basic_value().left().ok_or_else(|| anyhow!("Not a basic value"))
}

pub fn get_i64_type<'ctx>(cg_ctx: &CodeGenContext<'ctx>) -> Result<IntType<'ctx>> {
    Ok(cg_ctx.context.i64_type())
}

pub fn get_function_type<'ctx>(_cg_ctx: &CodeGenContext<'ctx>, arg_types: &[BasicMetadataTypeEnum<'ctx>], return_type: BasicTypeEnum<'ctx>) -> Result<FunctionType<'ctx>> {
    Ok(return_type.into_int_type().fn_type(arg_types, false))
}

pub fn add_function<'ctx>(cg_ctx: &CodeGenContext<'ctx>, function_name: &str, function_type: FunctionType<'ctx>) -> Result<FunctionValue<'ctx>> {
    Ok(cg_ctx.module.add_function(function_name, function_type, Some(Linkage::External)))
}

pub fn set_param_names<'ctx>(_cg_ctx: &CodeGenContext<'ctx>, function: FunctionValue<'ctx>, arg_names: &[&str]) -> Result<Vec<BasicValueEnum<'ctx>>> {
    let mut params = Vec::new();
    for (i, param) in function.get_param_iter().enumerate() {
        param.set_name(arg_names[i]);
        params.push(param.into());
    }
    Ok(params)
}

pub fn build_function<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, function: FunctionValue<'ctx>, body: impl FnOnce(&mut CodeGenContext<'ctx>, FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>>) -> Result<()> {
    let basic_block = cg_ctx.context.append_basic_block(function, "");
    cg_ctx.builder.position_at_end(basic_block);
    let ret: BasicValueEnum = body(cg_ctx, function)?.try_into().map_err(|_| anyhow!("Not a basic value"))?;
    cg_ctx.builder.build_return(Some(&ret))?;
    Ok(())
}

pub fn build_add<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, function: FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    let lhs = function.get_nth_param(0).ok_or_else(|| anyhow!("No param[0]"))?.into_int_value();
    let rhs = function.get_nth_param(1).ok_or_else(|| anyhow!("No param[1]"))?.into_int_value();
    Ok(cg_ctx.builder.build_int_add(lhs, rhs, "")?.into())
}

pub fn build_sub<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, function: FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    let lhs = function.get_nth_param(0).ok_or_else(|| anyhow!("No param[0]"))?.into_int_value();
    let rhs = function.get_nth_param(1).ok_or_else(|| anyhow!("No param[1]"))?.into_int_value();
    Ok(cg_ctx.builder.build_int_sub(lhs, rhs, "")?.into())
}

pub fn build_mul<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, function: FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    let lhs = function.get_nth_param(0).ok_or_else(|| anyhow!("No param[0]"))?.into_int_value();
    let rhs = function.get_nth_param(1).ok_or_else(|| anyhow!("No param[1]"))?.into_int_value();
    Ok(cg_ctx.builder.build_int_mul(lhs, rhs, "")?.into())
}

pub fn build_div<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, function: FunctionValue<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    let lhs = function.get_nth_param(0).ok_or_else(|| anyhow!("No param[0]"))?.into_int_value();
    let rhs = function.get_nth_param(1).ok_or_else(|| anyhow!("No param[1]"))?.into_int_value();
    Ok(cg_ctx.builder.build_int_signed_div(lhs, rhs, "")?.into())
}
