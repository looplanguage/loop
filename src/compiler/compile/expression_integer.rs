use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::integer::Integer;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_integer(compiler: &mut Compiler, int: Integer) -> CompilerResult {
    compiler.add_to_current_function(format!(".CONSTANT INT {};", int.value));

    CompilerResult::Success(Types::Basic(BaseTypes::Integer))
}
