use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::integer::Integer;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_integer(
    compiler: &mut Compiler,
    int: Integer,
) -> Result<Types, CompilerException> {
    compiler.add_to_current_function(format!(".CONSTANT INT {};", int.value));

    Ok(Types::Basic(BaseTypes::Integer))
}
