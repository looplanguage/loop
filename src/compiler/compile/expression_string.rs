use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_string(
    compiler: &mut Compiler,
    string: expression::string::LoopString,
) -> CompilerResult {
    let mut value = String::from(string.value);

    compiler.add_to_current_function(format!(".CONSTANT CHAR[] \"{}\";", value));

    CompilerResult::Success(Types::Basic(BaseTypes::String))
}
