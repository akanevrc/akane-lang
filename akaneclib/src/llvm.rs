use std::rc::Rc;
use anyhow::{
    anyhow,
    bail,
    Result,
};
use inkwell::{
    module::Linkage,
    types::{
        AnyTypeEnum,
        BasicTypeEnum,
        BasicMetadataTypeEnum,
        FloatType,
        FunctionType,
        IntType,
    },
    values::{
        AnyValue,
        AnyValueEnum,
        BasicValue,
        BasicValueEnum,
        BasicMetadataValueEnum,
        FloatValue,
        FunctionValue,
        IntValue,
    },
};
use crate::data::*;

pub fn get_const_from_cn<'ctx>(cg_ctx: &CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, cn: Rc<Cn>) -> Result<BasicValueEnum<'ctx>> {
    match cn.ty.borrow().clone() {
        ty if ty == TyKey::new_as_base("I64".to_owned()).get_val(sem_ctx).unwrap() =>
            get_const_i64(cg_ctx, cn.name.parse::<i64>().unwrap()).map(|value| value.as_basic_value_enum()),
        ty if ty == TyKey::new_as_base("F64".to_owned()).get_val(sem_ctx).unwrap() =>
            get_const_f64(cg_ctx, cn.name.parse::<f64>().unwrap()).map(|value| value.as_basic_value_enum()),
        _ =>
            bail!("Unsupported constant type: {}", cn.description()),
    }
}

pub fn get_const_i64<'ctx>(cg_ctx: &CodeGenContext<'ctx>, value: i64) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.context.i64_type().const_int(value as u64, false))
}

pub fn get_const_f64<'ctx>(cg_ctx: &CodeGenContext<'ctx>, value: f64) -> Result<FloatValue<'ctx>> {
    Ok(cg_ctx.context.f64_type().const_float(value))
}

pub fn build_call<'ctx>(cg_ctx: &CodeGenContext<'ctx>, function: FunctionValue<'ctx>, arguments: &[BasicMetadataValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>> {
    cg_ctx.builder.build_call(function, arguments, "")?
    .try_as_basic_value().left().ok_or_else(|| anyhow!("Not a basic value"))
}

pub fn get_type_from_ty<'ctx>(cg_ctx: &CodeGenContext<'ctx>, sem_ctx: &SemantizerContext, ty: Rc<Ty>) -> Result<AnyTypeEnum<'ctx>> {
    match ty {
        ty if ty == TyKey::new_as_base("I64".to_owned()).get_val(sem_ctx).unwrap() =>
            get_i64_type(cg_ctx).map(|ty| ty.into()),
        ty if ty == TyKey::new_as_base("F64".to_owned()).get_val(sem_ctx).unwrap() =>
            get_f64_type(cg_ctx).map(|ty| ty.into()),
        _ =>
            bail!("Unsupported type: {}", ty.description()),
    }
}

pub fn get_i64_type<'ctx>(cg_ctx: &CodeGenContext<'ctx>) -> Result<IntType<'ctx>> {
    Ok(cg_ctx.context.i64_type())
}

pub fn get_f64_type<'ctx>(cg_ctx: &CodeGenContext<'ctx>) -> Result<FloatType<'ctx>> {
    Ok(cg_ctx.context.f64_type())
}

pub fn get_function_type<'ctx>(_cg_ctx: &CodeGenContext<'ctx>, arg_types: &[BasicMetadataTypeEnum<'ctx>], return_type: BasicTypeEnum<'ctx>) -> Result<FunctionType<'ctx>> {
    match return_type {
        BasicTypeEnum::IntType(return_type) =>
            Ok(return_type.fn_type(arg_types, false)),
        BasicTypeEnum::FloatType(return_type) =>
            Ok(return_type.fn_type(arg_types, false)),
        _ =>
            bail!("Unsupported return type: {:?}", return_type),
    }
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

pub fn build_add<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: AnyValueEnum<'ctx>, rhs: AnyValueEnum<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    match (lhs, rhs) {
        (lhs, rhs) if lhs.is_int_value() && rhs.is_int_value() =>
            build_int_add(cg_ctx, lhs.into_int_value(), rhs.into_int_value()).map(|value| value.as_any_value_enum()),
        (lhs, rhs) if lhs.is_float_value() && rhs.is_float_value() =>
            build_float_add(cg_ctx, lhs.into_float_value(), rhs.into_float_value()).map(|value| value.as_any_value_enum()),
        (_, _) =>
            bail!("Unsupported type: {:?} + {:?}", lhs, rhs),
    }
}

pub fn build_sub<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: AnyValueEnum<'ctx>, rhs: AnyValueEnum<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    match (lhs, rhs) {
        (lhs, rhs) if lhs.is_int_value() && rhs.is_int_value() =>
            build_int_sub(cg_ctx, lhs.into_int_value(), rhs.into_int_value()).map(|value| value.as_any_value_enum()),
        (lhs, rhs) if lhs.is_float_value() && rhs.is_float_value() =>
            build_float_sub(cg_ctx, lhs.into_float_value(), rhs.into_float_value()).map(|value| value.as_any_value_enum()),
        (_, _) =>
            bail!("Unsupported type: {:?} - {:?}", lhs, rhs),
    }
}

pub fn build_mul<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: AnyValueEnum<'ctx>, rhs: AnyValueEnum<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    match (lhs, rhs) {
        (lhs, rhs) if lhs.is_int_value() && rhs.is_int_value() =>
            build_int_mul(cg_ctx, lhs.into_int_value(), rhs.into_int_value()).map(|value| value.as_any_value_enum()),
        (lhs, rhs) if lhs.is_float_value() && rhs.is_float_value() =>
            build_float_mul(cg_ctx, lhs.into_float_value(), rhs.into_float_value()).map(|value| value.as_any_value_enum()),
        (_, _) =>
            bail!("Unsupported type: {:?} * {:?}", lhs, rhs),
    }
}

pub fn build_div<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: AnyValueEnum<'ctx>, rhs: AnyValueEnum<'ctx>) -> Result<AnyValueEnum<'ctx>> {
    match (lhs, rhs) {
        (lhs, rhs) if lhs.is_int_value() && rhs.is_int_value() =>
            build_int_div(cg_ctx, lhs.into_int_value(), rhs.into_int_value()).map(|value| value.as_any_value_enum()),
        (lhs, rhs) if lhs.is_float_value() && rhs.is_float_value() =>
            build_float_div(cg_ctx, lhs.into_float_value(), rhs.into_float_value()).map(|value| value.as_any_value_enum()),
        (_, _) =>
            bail!("Unsupported type: {:?} / {:?}", lhs, rhs),
    }
}

pub fn build_int_add<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.builder.build_int_add(lhs, rhs, "")?)
}

pub fn build_int_sub<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.builder.build_int_sub(lhs, rhs, "")?)
}

pub fn build_int_mul<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.builder.build_int_mul(lhs, rhs, "")?)
}

pub fn build_int_div<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
    Ok(cg_ctx.builder.build_int_signed_div(lhs, rhs, "")?)
}

pub fn build_float_add<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: FloatValue<'ctx>, rhs: FloatValue<'ctx>) -> Result<FloatValue<'ctx>> {
    Ok(cg_ctx.builder.build_float_add(lhs, rhs, "")?)
}

pub fn build_float_sub<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: FloatValue<'ctx>, rhs: FloatValue<'ctx>) -> Result<FloatValue<'ctx>> {
    Ok(cg_ctx.builder.build_float_sub(lhs, rhs, "")?)
}

pub fn build_float_mul<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: FloatValue<'ctx>, rhs: FloatValue<'ctx>) -> Result<FloatValue<'ctx>> {
    Ok(cg_ctx.builder.build_float_mul(lhs, rhs, "")?)
}

pub fn build_float_div<'ctx>(cg_ctx: &mut CodeGenContext<'ctx>, lhs: FloatValue<'ctx>, rhs: FloatValue<'ctx>) -> Result<FloatValue<'ctx>> {
    Ok(cg_ctx.builder.build_float_div(lhs, rhs, "")?)
}
