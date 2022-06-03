use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::float::Float;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_float(compiler: &mut Compiler, flt: Float) -> CompilerResult {
    compiler.add_to_current_function(format!(".CONSTANT FLOAT {};", flt.value));

    CompilerResult::Success(Types::Basic(BaseTypes::Float))
}
