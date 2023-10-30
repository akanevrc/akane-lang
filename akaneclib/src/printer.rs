use anyhow::{
    anyhow,
    Result,
};
use crate::data::*;

pub fn print_to_file(path: &str, cg_ctx: &CodeGenContext) -> Result<()> {
    cg_ctx.module.print_to_file(path).map_err(|e| anyhow!("{}", e.to_string()))
}
