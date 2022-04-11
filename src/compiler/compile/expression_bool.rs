use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::boolean::Boolean;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_boolean(compiler: &mut Compiler, bl: Boolean) -> CompilerResult {
    if bl.value {
        compiler.add_to_current_function(String::from("true"));
    } else {
        compiler.add_to_current_function(String::from("false"));
    }

    CompilerResult::Success(Types::Basic(BaseTypes::Boolean))
}
