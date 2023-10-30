use anyhow::Result;
use crate::data::*;

pub fn generate(cg_ctx: &CodeGenContext, _sem_ctx: &SemantizerContext) -> Result<()> {
    let i64_type = cg_ctx.context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    let function = cg_ctx.module.add_function("sum", fn_type, None);
    let basic_block = cg_ctx.context.append_basic_block(function, "entry");
    cg_ctx.builder.position_at_end(basic_block);
    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();
    let z = function.get_nth_param(2).unwrap().into_int_value();
    let sum = cg_ctx.builder.build_int_add(x, y, "sum").unwrap();
    let sum = cg_ctx.builder.build_int_add(sum, z, "sum").unwrap();
    cg_ctx.builder.build_return(Some(&sum)).unwrap();
    Ok(())
}
