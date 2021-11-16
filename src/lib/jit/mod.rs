use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use std::mem;
use std::rc::Rc;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;

type DoubleFunc = unsafe extern "C" fn(u64) -> u64;

#[allow(dead_code)]
pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) compiled: Option<JitFunction<'ctx, DoubleFunc>>
}

// TODO: Document this quite a bit more, as this is a little complicated
impl<'ctx> CodeGen<'ctx> {
    #[allow(dead_code)]
    pub fn compile(&mut self) -> Option<bool> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into()], false);
        let function = self.module.add_function("double", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();

        let i64_double = i64_type.const_int(2, false);

        let doubled = self.builder.build_int_mul(x, i64_double, "doubled");

        self.builder.build_return(Some(&doubled));

        self.compiled = unsafe { self.execution_engine.get_function("double").ok() };

        None
    }

    #[allow(dead_code)]
    pub fn run(&self) {
        if let Some(compiled) = &self.compiled {
            let returned = unsafe { compiled.call(200) };

            println!("JIT RETURNED: {}", returned);
        }
    }
}
