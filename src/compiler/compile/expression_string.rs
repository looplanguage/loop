use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression;

pub fn compile_expression_string(
    compiler: &mut Compiler,
    string: expression::string::LoopString,
) -> CompilerResult {
    compiler.add_to_current_function(format!("\"{}\"", string.value));

    CompilerResult::Success
}
