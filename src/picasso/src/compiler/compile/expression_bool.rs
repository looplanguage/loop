use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::boolean::Boolean;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_boolean(
    compiler: &mut Compiler,
    bl: Boolean,
) -> Result<Types, CompilerException> {
    if bl.value {
        compiler.add_to_current_function(String::from(".CONSTANT BOOL true;"));
    } else {
        compiler.add_to_current_function(String::from(".CONSTANT BOOL false;"));
    }

    Ok(Types::Basic(BaseTypes::Boolean))
}
