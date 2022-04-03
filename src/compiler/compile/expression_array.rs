use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::array::Array;

pub fn compile_expression_array(compiler: &mut Compiler, arr: Array) -> CompilerResult {
    compiler.add_to_current_function("[".to_string());

    let mut index = 0;
    for value in arr.values.clone() {
        index += 1;

        compiler.compile_expression(*value.expression);

        if arr.values.len() > 1 && arr.values.len() != index {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    compiler.add_to_current_function("]".to_string());

    CompilerResult::Success
}
