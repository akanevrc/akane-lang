pub mod data;
pub mod lexer;
pub mod parser;
pub mod semantizer;
pub mod codegen;
pub mod printer;
mod macros;

use std::fs;
use anyhow::Error;
use inkwell::context::Context;
use crate::data::*;

pub fn compile(in_path: &str, out_path: &str) -> Result<(), Vec<Error>> {
    let code = fs::read_to_string(in_path).map_err(|e| vec![Error::from(e)])?;
    let tokens = lexer::lex(&code)?;
    let mut asts = parser::parse(tokens)?;
    let mut sem_ctx = SemantizerContext::new();
    semantizer::semantize(&mut sem_ctx, &mut asts)?;
    let ctx = Context::create();
    let mut cg_ctx = CodeGenContext::new(&ctx);
    codegen::generate(&mut cg_ctx, &sem_ctx).map_err(|e| vec![e])?;
    printer::print_to_file(out_path, &cg_ctx).map_err(|e| vec![e])?;
    Ok(())
}
