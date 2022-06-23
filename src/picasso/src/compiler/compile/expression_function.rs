use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::types::{BaseTypes, FunctionType, Types};
use rand::random;

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
    compiler.function_count += 1;
    let random_identifier: i64 = random();
    let mut function_type: Types;
    // (Transpiled, Named, Index)
    let mut named_function: Option<(String, String, u32)> = None;

    // Named function ^.^
    if !func.name.is_empty() {
        let mut parameters: Vec<Parameter> = Vec::new();

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

        let mut type_parameters: Vec<Types> = Vec::new();

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

        // TODO: Return type is auto for now, but types for parameters is et
        function_type = Types::Function(FunctionType {
            return_type: Box::from(Types::Auto),
            parameter_types: type_parameters,
            reference: format!("local::{}", func.name),
            is_method: false,
        });

        let var = compiler.define_variable(func.name.clone(), function_type.clone(), -1);

        named_function = Option::from((format!("var_{}", var.index), var.name.clone(), var.index));
    }

    if let Some(predefined) = func.predefined_type {
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
    for (index, parameter) in func.parameters.iter().enumerate() {
        let mut param_type = parameter._type.clone();

        if let Types::Basic(BaseTypes::UserDefined(name)) = &param_type {
            if let Some(compound) = compiler.get_compound_type(name) {
                param_type = compound.clone()
            } else {
                return CompilerResult::Exception(CompilerException::UnknownType(name.clone()));
            }
        }

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

    function_type = if named_function.is_some() {
        Types::Function(FunctionType {
            return_type: Box::from(return_type),
            parameter_types,
            reference: format!("local::{}", func.name),
            is_method: false,
        })
    } else {
        Types::Function(FunctionType {
            return_type: Box::from(return_type),
            parameter_types,
            reference: "".to_string(),
            is_method: false,
        })
    };

    compiler.exit_variable_scope();
    if !func.name.is_empty() {
        let named_function = named_function.unwrap();

        let variable = compiler
            .variable_scope
            .as_ref()
            .borrow_mut()
            .get_variable_mutable(named_function.2, named_function.1);

        if let Some(variable) = variable {
            variable.as_ref().borrow_mut()._type = function_type.clone();
        }
    }

    CompilerResult::Success(function_type)
}
