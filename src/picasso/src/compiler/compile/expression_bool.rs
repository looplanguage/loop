use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::boolean::Boolean;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_boolean(compiler: &mut Compiler, bl: Boolean) -> CompilerResult {
    if bl.value {
        compiler.add_to_current_function(String::from(".CONSTANT BOOL true;"));
    } else {
        compiler.add_to_current_function(String::from(".CONSTANT BOOL false;"));
    }

    CompilerResult::Success(Types::Basic(BaseTypes::Boolean))
}
