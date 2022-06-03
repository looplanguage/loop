use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::expression::function::{Call, Function, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::types::{BaseTypes, ClassItemType, Compound, Types};

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    // This is for calling functions from a library & instantiating classes
    // First class instantation
    if let expression::Expression::Identifier(i) = *call.clone().identifier {
        // Check if this is a class
        let class = compiler.get_compound_type(&i.value);

        if let Some(Types::Compound(class_type)) = class {
            if let Compound(name, mut values) = class_type.clone() {
                // Instantiate the class using a constant
                compiler.add_to_current_function(format!(".CONSTANT {} {{", name));
                for (index, mut value) in (*values).iter_mut().enumerate() {
                    // Define "self" if its a function
                    let result = if let Expression::Function(func) = value.1.value.clone() {
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
                        compiler.compile_expression(value.1.value.clone())
                    };

                    if result.is_exception() {
                        return result;
                    }

                    value.1.index = index as u32;
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
    }

    // This is for calling functions defined in Loop
    compiler.add_to_current_function(".CALL {".to_string());

    let mut method_type: Option<Types> = None;
    let mut class_reference: Option<Expression> = None;

    if let Expression::String(ref namespace) = *call.identifier {
        let split: Vec<&str> = namespace.value.split("::").collect();

        if split.len() > 1 {
            let name = split.first().unwrap().to_string();
            let method = split.get(1).unwrap().to_string();

            class_reference = Some(Expression::Identifier(Identifier { value: name }));

            let result = compiler.compile_expression(Expression::Index(Box::new(Index {
                left: class_reference.clone().unwrap(),
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

        method_type = Some(Types::Function(func_signature.clone()));
    }

    compiler.add_to_current_function(String::from("} {"));

    // Insert "self" parameter
    if let Some(class_reference) = class_reference {
        compiler.compile_expression(class_reference);
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
