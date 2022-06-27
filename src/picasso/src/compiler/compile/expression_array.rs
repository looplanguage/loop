use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::array::Array;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_array(
    compiler: &mut Compiler,
    arr: Array,
) -> Result<Types, CompilerException> {
    let mut array_type: Types = Types::Array(Box::from(Types::Basic(BaseTypes::Integer)));

    if !arr.values.is_empty() {
        compiler.add_to_current_function(format!(".CONSTANT {} [", array_type.transpile()));

        let mut index = 0;
        for value in arr.values.clone() {
            index += 1;

            let result = compiler.compile_expression(*value.expression)?;

            if index == 1 {
                array_type = Types::Array(Box::from(result));
            }

            if arr.values.len() > 1 && arr.values.len() != index {
                compiler.add_to_current_function("".to_string());
            }
        }

        compiler.add_to_current_function("];".to_string());
    } else if let Types::Array(_) = array_type {
        compiler.add_to_current_function(format!(".CONSTANT {} [];", array_type.transpile()));
    }

    Ok(array_type)
}
