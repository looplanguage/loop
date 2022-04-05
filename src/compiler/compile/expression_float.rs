use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::float::Float;

pub fn compile_expression_float(compiler: &mut Compiler, flt: Float) -> CompilerResult {
    compiler.add_to_current_function(format!("{}f", flt.value));

    CompilerResult::Success
}
