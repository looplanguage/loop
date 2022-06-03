use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::expression::function::Call;
use crate::parser::types::{Compound, Types};

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    // This is for calling functions from a library & instantiating classes
    // First class instantation
    if let expression::Expression::Identifier(i) = *call.clone().identifier {
        // Check if this is a class
        let class = compiler.get_compound_type(&i.value);

        if let Some(Types::Compound(Compound(name, mut values))) = class {
            // Instantiate the class using a constant
            compiler.add_to_current_function(format!(".CONSTANT {} {{", name));
            for (index, mut value) in (*values).iter_mut().enumerate() {
                compiler.compile_expression(value.1 .1 .1.clone());
                value.1 .0 = index as u32;
            }

            compiler.add_to_current_function("};".to_string());

            return CompilerResult::Success(Types::Compound(Compound(name, values)));
        }
    // Second library function calling
    } else if let expression::Expression::String(namespace) = *call.clone().identifier {
        let splitted_namespace: Vec<&str> = namespace.value.split("::").collect();
        let lib_name = splitted_namespace[0].to_string();

        // Checks whether file is imported
        if compiler.imports.contains(&lib_name) {
            compiler.add_to_current_function(".CALL ".to_string());
            compiler.add_to_current_function(namespace.value);
            compiler.add_to_current_function(" { ".to_string());
            for parameter in call.parameters {
                let result = compiler.compile_expression(parameter);

                if let CompilerResult::Exception(_) = &result {
                    return result;
                }
            }
            compiler.add_to_current_function(String::from("};"));

            // Since we do not know what the return type of the function is, we use Types::Auto
            // TODO: You do know function signiture it is in the header or "function_signiture" function
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

        if let CompilerResult::Exception(_) = &result {
            return result;
        }
    }

    compiler.add_to_current_function(String::from("};"));

    CompilerResult::Success(*func_signature.return_type.clone())
}
