use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::array::Array;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_array(compiler: &mut Compiler, arr: Array) -> CompilerResult {
    let mut array_type: Types = Types::Array(BaseTypes::Integer);

    if !arr.values.is_empty() {
        compiler.add_to_current_function("[".to_string());

        let mut index = 0;
        for value in arr.values.clone() {
            index += 1;

            let result = compiler.compile_expression(*value.expression, false);

            if let CompilerResult::Success(_type) = result {
                if index == 1 {
                    if let Types::Basic(basic) = _type {
                        array_type = Types::Array(basic);
                    }
                }

                if arr.values.len() > 1 && arr.values.len() != index {
                    compiler.add_to_current_function(", ".to_string());
                }
            } else {
                return result;
            }
        }

        compiler.add_to_current_function("].to!(Variant[])".to_string());
    } else {
        compiler.add_to_current_function("(cast(Variant[])[])".to_string());
    }

    CompilerResult::Success(array_type)
}
