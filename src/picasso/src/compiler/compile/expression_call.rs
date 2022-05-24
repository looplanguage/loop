use std::fmt::format;
use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::expression::function::Call;
use crate::parser::types::Types;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    // This is for calling functions from a library & instantiating classes
    if let expression::Expression::Identifier(i) = *call.clone().identifier {
        // Check if this is a class
        let class = compiler.get_compound_type(&i.value);

        if let Some(class) = class {
            if let Types::Compound(name, values) = class {
                // Instantiate the class using a constant
                compiler.add_to_current_function(format!(".CONSTANT {} {{", name));
                let mut cloned_values = values.clone();

                let mut index = 0;
                for value in &mut *cloned_values {
                    compiler.compile_expression(value.1.1.1.clone());
                    value.1.0 = index;

                    index += 1;
                }

                compiler.add_to_current_function("};".to_string());

                return CompilerResult::Success(Types::Compound(name.clone(), cloned_values))
            }
        }

        let x: Vec<&str> = i.value.split("::").collect();
        let name = x[0].to_string();
        if compiler.imports.contains(&name) {
            compiler.add_to_current_function(".CALL ".to_string());
            compiler.add_to_current_function(i.value);
            compiler.add_to_current_function(" { ".to_string());
            for parameter in call.parameters {
                let result = compiler.compile_expression(parameter);

                #[allow(clippy::single_match)]
                match &result {
                    CompilerResult::Exception(_exception) => return result,
                    _ => (),
                }
            }
            compiler.add_to_current_function(String::from("};"));

            // Since we do not know what the return type of the function is, we use Types::Auto
            return CompilerResult::Success(Types::Auto);
        }
    }

    // This is for calling functions defined in Loop
    compiler.add_to_current_function(".CALL {".to_string());
    let result = compiler.compile_expression(*call.identifier);

    let func_signature = match &result {
        CompilerResult::Exception(_exception) => return result,
        CompilerResult::Success(_type) => {
            if let Types::Function(func) = _type {
                func
            } else {
                return CompilerResult::Exception(CompilerException::CallingNonFunction(
                    _type.transpile(),
                ));
            }
        }
        _ => return CompilerResult::Exception(CompilerException::Unknown),
    };

    compiler.add_to_current_function(String::from("} {"));

    for parameter in call.parameters {
        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }
    }

    compiler.add_to_current_function(String::from("};"));

    CompilerResult::Success(*func_signature.return_type.clone())
}
