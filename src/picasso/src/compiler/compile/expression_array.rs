use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::array::Array;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_array(compiler: &mut Compiler, arr: Array) -> CompilerResult {
    let mut array_type: Types = Types::Array(Box::from(Types::Basic(BaseTypes::Integer)));

    if !arr.values.is_empty() {
        compiler.add_to_current_function(format!(".CONSTANT {} [", array_type.transpile()));

        let mut index = 0;
        for value in arr.values.clone() {
            index += 1;

            let result = compiler.compile_expression(*value.expression);

            if let CompilerResult::Success(_type) = result {
                if index == 1 {
                    array_type = Types::Array(Box::from(_type.clone()));
                }

                if arr.values.len() > 1 && arr.values.len() != index {
                    compiler.add_to_current_function("".to_string());
                }
            } else {
                return result;
            }
        }

        compiler.add_to_current_function("];".to_string());
    } else if let Types::Array(_) = array_type {
        compiler.add_to_current_function(format!(".CONSTANT {} [];", array_type.transpile()));
    }

    CompilerResult::Success(array_type)
}
