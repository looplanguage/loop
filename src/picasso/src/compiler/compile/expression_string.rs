use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_string(
    compiler: &mut Compiler,
    string: expression::string::LoopString,
) -> Result<Types, CompilerException> {
    compiler.add_to_current_function(format!(".CONSTANT CHAR[] \"{}\";", string.value));

    Ok(Types::Basic(BaseTypes::String))
}
