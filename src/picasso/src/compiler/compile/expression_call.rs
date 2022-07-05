use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::function::{Call, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Compound, Types};

pub fn compile_expression_call(
    compiler: &mut Compiler,
    call: Call,
) -> Result<Types, CompilerException> {
    // This is for calling functions from a library & instantiating classes
    // First class instantation from the current module
    if let Expression::Identifier(i) = *call.clone().identifier {
        // Check if this is a class
        let class = compiler.get_compound_type(&i.value);

        if let Some(Types::Compound(class_type)) = class {
            let idenfitier = compiler.resolve_symbol(&i.value);
            let Compound(name, values) = class_type.clone();

            if let Some(definition) = idenfitier {
                // Wrapped in a call expression so that we can do more during execution
                compiler.add_to_current_function(format!(
                    ".CALL {{ .FUNCTION \"\" {} {} ARGUMENTS {{}} FREE {{}} THEN {{",
                    compiler.function_count,
                    definition.transpile()
                ));

                compiler.function_count += 1;

                let temp_var = compiler.define_symbol(
                    "temporary_class_holder".to_string(),
                    Types::Compound(class_type.clone()),
                    0,
                );

                // Instantiate the class using a constant and store it into the temporary value
                compiler.add_to_current_function(format!(
                    ".STORE {} {{ .CONSTANT {} {{",
                    temp_var.index,
                    definition.transpile()
                ));

                for value in &*values {
                    // Define "self" if its a function
                    if let Expression::Function(func) = value.value.clone() {
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

                        compiler.compile_expression(Expression::Function(func))?
                    } else {
                        compiler.compile_expression(value.value.clone())?
                    };
                }

                compiler.add_to_current_function("};};".to_string());

                let found = values.iter().find(|item| item.name == "constructor");

                if let Some(constructor) = found {
                    // TODO: Explain this a bit better, probably needs some refactoring anyway
                    compiler.add_to_current_function(format!(".CALL {{ .INDEX {{ .LOAD VARIABLE {}; }} {{ .CONSTANT INT {}; }}; }} {{ .LOAD VARIABLE {}; ", temp_var.index, constructor.index, temp_var.index));

                    // Compile parameters
                    for parameter in call.parameters {
                        compiler.compile_expression(parameter)?;
                    }

                    compiler.add_to_current_function("};".to_string())
                }

                compiler.add_to_current_function(format!(
                    ".RETURN {{ .LOAD VARIABLE {}; }};",
                    temp_var.index
                ));

                // End of variable definition, function definition & call to it
                compiler.add_to_current_function("};} {};".to_string());

                return Ok(Types::Compound(Compound(name, values)));
            }
        }
    } else if let Expression::String(namespace) = *call.clone().identifier {
        let lib_name = "".to_string();

        // Checks whether file is imported
        if compiler.imports.contains(&lib_name) {
            compiler.add_to_current_function(".CALL ".to_string());
            compiler.add_to_current_function(namespace.value);
            compiler.add_to_current_function(" { ".to_string());

            for parameter in call.parameters {
                compiler.compile_expression(parameter)?;
            }

            compiler.add_to_current_function(String::from("};"));

            // Since we do not know what the return type of the function is, we use Types::Auto
            // TODO: You do know function signiture it is in the header or "function_signiture" function
            return Ok(Types::Auto);
        }
    }

    if let Expression::Index(a) = *call.identifier.clone() {
        if let Expression::Identifier(ident) = a.index {
            let value = ident.value;

            if let Expression::Identifier(library) = &a.left {
                if compiler.imports.contains(&library.value) {
                    compiler.add_to_current_function(".CALL ".to_string());
                    compiler.add_to_current_function(format!("{}::{}", library.value, value));
                    compiler.add_to_current_function(" { ".to_string());
                    for parameter in call.parameters {
                        compiler.compile_expression(parameter)?;
                    }
                    compiler.add_to_current_function(String::from("};"));

                    // Since we do not know what the return type of the function is, we use Types::Auto
                    // TODO: You do know function signiture it is in the header or "function_signiture" function
                    return Ok(Types::Auto);
                }
            }

            compiler.drier();
            let result = compiler.compile_expression(a.left.clone());
            compiler.undrier();

            if let Ok(succ) = result {
                match &*value {
                    "push" => {
                        for parameter in &call.parameters {
                            compiler.add_to_current_function(".PUSH {".to_string());
                            compiler.compile_expression(a.left.clone())?;
                            compiler.add_to_current_function("} { ".to_string());
                            compiler.compile_expression(parameter.clone())?;
                            compiler.add_to_current_function("};".to_string());
                        }

                        return Ok(succ);
                    }
                    "remove" => {
                        for parameter in &call.parameters {
                            compiler.add_to_current_function(".POP {".to_string());
                            compiler.compile_expression(a.left.clone())?;
                            compiler.add_to_current_function("} { ".to_string());
                            compiler.compile_expression(parameter.clone())?;
                            compiler.add_to_current_function("};".to_string());
                        }

                        return Ok(succ);
                    }
                    _ => {}
                }
            }
        }
    }

    let mut index = None;
    let mut self_reference: Option<Expression> = None;

    if let Expression::Index(i) = *call.identifier.clone() {
        self_reference = Some(i.left.clone());

        if let Expression::Identifier(ident) = i.index {
            index = Some(ident.value);
        }
    }

    // Catching the build extesion function "len"
    // "[1, 2, 3, 4, 5].len() == 5" this is true
    if let Some(name) = &index {
        if name == "len" {
            if let Some(self_reference) = self_reference.clone() {
                compiler.drier();
                let result = compiler.compile_expression(self_reference.clone());
                // Should prob not be here, but I did not know how to make clippy happy
                match result {
                    Ok(_type) => {
                        // Only strings and arrays have a length
                        match _type {
                            Types::Compound(_)
                            | Types::Basic(_)
                            | Types::Auto
                            | Types::Function(_)
                            | Types::Module(_)
                            | Types::Void
                            | Types::Library(_) => {
                                if _type != Types::Basic(BaseTypes::String) {
                                    return Err(CompilerException::WrongType(
                                        "array or string".to_string(),
                                        format!("{}", _type),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(exception) => {
                        return Err(exception);
                    }
                }
                compiler.undrier();
                compiler.add_to_current_function(".LENGTH { ".to_string());
                let result = compiler.compile_expression(self_reference);
                compiler.add_to_current_function("};".to_string());
                if result.is_err() {
                    return result;
                } else {
                    return Ok(Types::Basic(BaseTypes::Integer));
                }
            } else {
                panic!("HELLO< SHOLD NOT PANIC")
            }
        }
    }

    // Check if self reference exists and points to a module
    if let Some(self_reference) = self_reference.clone() {
        compiler.drier();
        let result = compiler.compile_expression(self_reference);
        compiler.undrier();

        if let Ok(Types::Module(module)) = result {
            // This errors because its too late and we already generated a bunch of code
            return compile_expression_call(
                compiler,
                Call {
                    identifier: Box::new(Expression::Identifier(Identifier {
                        value: format!("{}::{}", module, index.unwrap()),
                    })),
                    parameters: call.parameters,
                },
            );
        }
    }

    // This is for calling functions defined in Loop
    compiler.add_to_current_function(".CALL {".to_string());

    let mut method_type: Option<Types> = None;

    if let Expression::String(ref namespace) = *call.identifier {
        let split: Vec<&str> = namespace.value.split("::").collect();

        if split.len() > 1 {
            let name = split.first().unwrap().to_string();
            let method = split.get(1).unwrap().to_string();

            let _type = compiler.compile_expression(Expression::Index(Box::new(Index {
                left: Expression::Identifier(Identifier {
                    value: name.clone(),
                }),
                index: Expression::Identifier(Identifier { value: method }),
            })))?;

            if let Types::Function(func) = _type.clone() {
                if func.is_method {
                    self_reference = Some(Expression::Identifier(Identifier { value: name }));
                }
            }

            method_type = Some(_type)
        }
    }

    if method_type.is_none() {
        // If left side of the call was an index, this is a method being called on it so insert
        // "self"

        let _type = compiler.compile_expression(*call.identifier.clone())?;
        let func_signature = match _type {
            Types::Function(func) => func,
            _ => {
                return Err(CompilerException::CallingNonFunction(_type.transpile()));
            }
        };

        method_type = Some(*func_signature.return_type);
    }

    compiler.add_to_current_function(String::from("} {"));

    // Insert "self" parameter
    if let Some(self_reference) = self_reference {
        compiler.compile_expression(self_reference)?;
    }

    for parameter in call.parameters {
        compiler.compile_expression(parameter)?;
    }

    compiler.add_to_current_function(String::from("};"));

    Ok(method_type.unwrap())
}
