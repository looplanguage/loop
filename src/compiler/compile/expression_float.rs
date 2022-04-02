use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::object::float;
use crate::lib::object::Object;
use crate::parser::expression::float::Float;

pub fn compile_expression_float(compiler: &mut Compiler, flt: Float) -> CompilerResult {
    compiler.add_to_current_function(format!("{}f", flt.value));

    CompilerResult::Success
}
