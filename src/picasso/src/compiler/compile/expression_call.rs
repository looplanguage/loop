use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::expression::function::{Call, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;
use crate::parser::types::{Compound, Types};

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    // This is for calling functions from a library & instantiating classes
    // First class instantation
    if let expression::Expression::Identifier(i) = *call.clone().identifier {
        // Check if this is a class
        let class = compiler.get_compound_type(&i.value);

        if let Some(Types::Compound(class_type)) = class {
            let Compound(name, values) = class_type.clone();
            // Wrapped in a call expression so that we can do more during execution
            compiler.add_to_current_function(format!(
                ".CALL {{ .FUNCTION \"\" {} {} ARGUMENTS {{}} FREE {{}} THEN {{",
                compiler.function_count, i.value
            ));

            compiler.function_count += 1;

            let temp_var = compiler.define_variable(
                "temporary_class_holder".to_string(),
                Types::Compound(class_type.clone()),
                0,
            );

            // Instantiate the class using a constant and store it into the temporary value
            compiler.add_to_current_function(format!(
                ".STORE {} {{ .CONSTANT {} {{",
                temp_var.index, name
            ));

            for value in &*values {
                // Define "self" if its a function
                let result = if let Expression::Function(func) = value.value.clone() {
                    let mut func = func.clone();

                    func.parameters.insert(
                        0,
                        Parameter {
                            identifier: Identifier {
                                value: "self".to_string(),
                            },
                            _type: Types::Compound(class_type.clone()),
                        },
                    );

                    compiler.compile_expression(Expression::Function(func))
                } else {
                    compiler.compile_expression(value.value.clone())
                };

                if result.is_exception() {
                    return result;
                }
            }

            compiler.add_to_current_function("};};".to_string());

            let found = values.iter().find(|item| item.name == "constructor");

            if let Some(constructor) = found {
                // TODO: Explain this a bit better, probably needs some refactoring anyway
                compiler.add_to_current_function(format!(".CALL {{ .INDEX {{ .LOAD VARIABLE {}; }} {{ .CONSTANT INT {}; }}; }} {{ .LOAD VARIABLE {}; ", temp_var.index, constructor.index, temp_var.index));

                // Compile parameters
                for parameter in call.parameters {
                    let result = compiler.compile_expression(parameter);

                    if let CompilerResult::Exception(_) = &result {
                        return result;
                    }
                }

                compiler.add_to_current_function("};".to_string())
            }

            compiler.add_to_current_function(format!(
                ".RETURN {{ .LOAD VARIABLE {}; }};",
                temp_var.index
            ));

            // End of variable definition, function definition & call to it
            compiler.add_to_current_function("};} {};".to_string());

            return CompilerResult::Success(Types::Compound(Compound(name, values)));

            // Second library function calling
        }
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

    let mut method_type: Option<Types> = None;
    let mut self_reference: Option<Expression> = None;

    if let Expression::String(ref namespace) = *call.identifier {
        let split: Vec<&str> = namespace.value.split("::").collect();

        if split.len() > 1 {
            let name = split.first().unwrap().to_string();
            let method = split.get(1).unwrap().to_string();

            self_reference = Some(Expression::Identifier(Identifier { value: name }));

            let result = compiler.compile_expression(Expression::Index(Box::new(Index {
                left: self_reference.clone().unwrap(),
                index: Expression::Identifier(Identifier { value: method }),
            })));

            if let CompilerResult::Success(_type) = result {
                method_type = Some(_type)
            } else {
                return result;
            }
        }
    }

    if method_type.is_none() {
        // If left side of the call was an index, this is a method being called on it so insert
        // "self"

        if let Expression::Index(i) = *call.identifier.clone() {
            self_reference = Some(i.left.clone());
        }

        let result = compiler.compile_expression(*call.identifier.clone());

        let func_signature = match &result {
            CompilerResult::Exception(_exception) => return result,
            CompilerResult::Success(_type) => match _type {
                Types::Function(func) => func,
                _ => {
                    return CompilerResult::Exception(CompilerException::CallingNonFunction(
                        _type.transpile(),
                    ));
                }
            },
            _ => return CompilerResult::Exception(CompilerException::Unknown),
        };

        method_type = Some(*func_signature.clone().return_type);
    }

    compiler.add_to_current_function(String::from("} {"));

    // Insert "self" parameter
    if let Some(self_reference) = self_reference {
        compiler.compile_expression(self_reference);
    }

    for parameter in call.parameters {
        let result = compiler.compile_expression(parameter);

        if let CompilerResult::Exception(_) = &result {
            return result;
        }
    }

    compiler.add_to_current_function(String::from("};"));

    CompilerResult::Success(method_type.unwrap())
}
