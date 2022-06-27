use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_null(_compiler: &mut Compiler) -> Result<Types, CompilerException> {
    _compiler.add_to_current_function(".CONSTANT VOID;".to_string());

    Ok(Types::Basic(BaseTypes::Integer))
}
