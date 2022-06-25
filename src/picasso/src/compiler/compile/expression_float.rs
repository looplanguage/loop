use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::float::Float;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_float(
    compiler: &mut Compiler,
    flt: Float,
) -> Result<Types, CompilerException> {
    compiler.add_to_current_function(format!(".CONSTANT FLOAT {};", flt.value));

    Ok(Types::Basic(BaseTypes::Float))
}
