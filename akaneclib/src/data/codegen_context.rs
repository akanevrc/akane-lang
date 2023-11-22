use std::{
    cell::RefCell,
    rc::Rc,
};
use inkwell::{
    OptimizationLevel,
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    values::{
        AnyValueEnum,
        FunctionValue,
    },
};
use crate::data::*;

pub struct CodeGenContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
    pub bound_values: GenericStore<String, AnyValueEnum<'ctx>>,
    pub functions: GenericStore<String, FunctionValue<'ctx>>,
    pub ty_env_stack: Vec<Rc<RefCell<TyEnv>>>,
}

impl<'ctx> CodeGenContext<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        Self {
            context,
            module,
            builder,
            execution_engine,
            bound_values: GenericStore::new(),
            functions: GenericStore::new(),
            ty_env_stack: Vec::new(),
        }
    }
}
