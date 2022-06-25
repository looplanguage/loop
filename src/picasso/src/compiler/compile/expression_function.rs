use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::types::{BaseTypes, FunctionType, Types};

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub code: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Types,
}

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: Types,
}

pub fn compile_expression_function(
    compiler: &mut Compiler,
    func: expression::function::Function,
) -> CompilerResult {
    // Increase the function counter to keep getting unique identifiers
    compiler.function_count += 1;

    // Then we save this unique identifier for later use when we need it to assign a type to the
    // function return value.
    let random_identifier: i64 = compiler.function_count as i64;

    // A function can optionally be named or anonymous(lambda). This is a tuple with the data that
    // it needs
    // (Transpiled, Named, Index)
    let mut named_function: Option<(String, String, u32)> = None;

    // Named functions are compiled differently
    if !func.name.is_empty() {
        let mut parameters: Vec<Parameter> = Vec::new();

        // Loop through all the parameters to check if no duplicate names exist and push them to the
        // parameters vector to be used to check their type further down
        for parameter in &func.parameters {
            if parameters
                .iter()
                .any(|p| p.name == parameter.identifier.value)
            {
                return CompilerResult::Exception(CompilerException::DoubleParameterName(
                    parameter.identifier.value.clone(),
                ));
            }

            let parameter = Parameter {
                name: parameter.identifier.value.clone(),
                parameter_type: parameter._type.clone(),
            };

            parameters.push(parameter);
        }

        // Type parameters are used to declare the full type specification for this function
        let mut type_parameters: Vec<Types> = Vec::new();

        // Loop through the parameters we extracted above and get their type
        for parameter in &parameters {
            let mut param_type = parameter.parameter_type.clone();

            if let Types::Basic(BaseTypes::UserDefined(name)) = &param_type {
                if let Some(compound) = compiler.get_compound_type(name) {
                    param_type = compound.clone()
                } else {
                    return CompilerResult::Exception(CompilerException::UnknownType(name.clone()));
                }
            }

            type_parameters.push(param_type)
        }

        // The return type here is "Auto" as we infer it later when we compile the function body
        let function_type = Types::Function(FunctionType {
            return_type: Box::from(Types::Auto),
            parameter_types: type_parameters,
            reference: format!("local::{}", func.name),
            is_method: false,
        });

        // Define it with the current type and set "named_function" which we use later as well
        let var = compiler.define_variable(func.name.clone(), function_type.clone(), -1);
        named_function = Option::from((format!("var_{}", var.index), var.name.clone(), var.index));
    }

    // Check if during parsing a function had its type pre-defined, this is only the case in methods
    // for classes.
    if let Some(predefined) = func.predefined_type {
        // If it is pre-defined, just use that type in Arc generation
        compiler.add_to_current_function(format!(
            ".FUNCTION \"{}\" {} {} ARGUMENTS {{",
            named_function
                .clone()
                .unwrap_or(("".to_string(), "".to_string(), 0))
                .0,
            compiler.function_count,
            predefined.transpile()
        ));
    } else {
        // If we don't know the type we instead use a placeholder and use the previously defined
        // "random_identifier"
        compiler.add_to_current_function(format!(
            ".FUNCTION \"{}\" {} REPLACE_TYPE_{} ARGUMENTS {{",
            named_function
                .clone()
                .unwrap_or(("".to_string(), "".to_string(), 0))
                .0,
            compiler.function_count,
            random_identifier
        ));
    }

    let mut parameter_types: Vec<Types> = Vec::new();

    compiler.enter_variable_scope();
    // Here we go through all the parameters again, the first reason because we didn't do so yet for
    // unnamed functions and secondly we do it for named functions as well as we now define the
    // parameters as variables otherwise their names would be unknown
    for (index, parameter) in func.parameters.iter().enumerate() {
        let mut param_type = parameter._type.clone();

        if let Types::Basic(BaseTypes::UserDefined(name)) = &param_type {
            if let Some(compound) = compiler.get_compound_type(name) {
                param_type = compound.clone()
            } else {
                return CompilerResult::Exception(CompilerException::UnknownType(name.clone()));
            }
        }

        // Define the parameter in the current scope, before compiling the function body so
        // parameters can be used inside the body
        compiler.define_variable(parameter.identifier.value.clone(), param_type, index as i32);

        let _type = parameter.get_type();

        parameter_types.push(parameter._type.clone());

        // Try to find it
        let found = compiler.resolve_variable(&_type);

        if let Some(found) = found {
            compiler.add_to_current_function(format!("{};", found.transpile()));
        } else {
            compiler.add_to_current_function(format!("{};", _type));
        }
    }

    compiler.add_to_current_function("} FREE {} THEN ".to_string());

    // Now we compile the function body
    let result = compiler.compile_block(func.body, func.name.is_empty());

    compiler.add_to_current_function(";".to_string());

    // Currently infer and dont allow manually setting type
    let return_type = {
        if let CompilerResult::Success(_type) = result {
            _type
        } else if let CompilerResult::Exception(exception) = result {
            return CompilerResult::Exception(exception);
        } else {
            Types::Void
        }
    };

    // Set return type of named function, if it exists
    let return_name = {
        if let Some(var) = compiler.resolve_variable(&return_type.transpile()) {
            var.transpile()
        } else {
            return_type.transpile()
        }
    };

    compiler
        .replace_at_current_function(format!("REPLACE_TYPE_{}", random_identifier), return_name);

    // Set the new function type
    let function_type = Types::Function(FunctionType {
        return_type: Box::from(return_type),
        parameter_types,
        reference: if named_function.is_some() { format!("local::{}", func.name) } else { "".to_string() },
        is_method: false,
    });

    compiler.exit_variable_scope();

    // If the function was named, we set the return type of the function to inferred type from the
    // body compilation.
    if !func.name.is_empty() {
        let named_function = named_function.unwrap();

        let variable = compiler.get_variable_mutable(named_function.2, named_function.1, None);

        if let Some(variable) = variable {
            variable.as_ref().borrow_mut()._type = function_type.clone();
            variable.as_ref().borrow_mut().modifiers.public = func.public;
        }
    }

    CompilerResult::Success(function_type)
}
