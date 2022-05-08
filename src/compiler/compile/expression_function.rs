use crate::compiler::{Compiler, CompilerResult};
use crate::compiler::variable_table::Variable;
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::types::{FunctionType, Types};

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

        let mut type_parameters: Vec<Box<Types>> = Vec::new();

        for parameter in &parameters {
            type_parameters.push(Box::from(parameter.parameter_type.clone()))
        }

        // TODO: Return type is auto for now, but types for parameters is et
        function_type = Types::Function(FunctionType {
            return_type: Box::from(Types::Auto),
            parameter_types: type_parameters,
            reference: format!("local::{}", func.name)
        });

        let var = compiler.define_variable(
            format!("{}{}", compiler.location, func.name),
            function_type.clone(),
            -1,
        );

        named_function = Option::from((var.transpile(), var.name.clone(), var.index));
    }

    compiler.add_to_current_function(format!(".FUNCTION \"{}\" REPLACE_TYPE_{} ARGUMENTS {{", func.name, func.name));

    let mut parameter_types: Vec<Box<Types>> = Vec::new();

    let mut index = 0;
    for parameter in &func.parameters {
        let symbol = compiler.define_variable(
            format!(
                "{}{}",
                compiler.location,
                parameter.identifier.value.clone(),
            ),
            parameter._type.clone(),
            index,
        );

        let _type = parameter.get_type();

        parameter_types.push(Box::from(parameter._type.clone()));


        compiler.add_to_current_function(format!("{};", _type));

        index += 1;
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
    function_type = if let Some(named_function) = named_function.clone() {
        compiler.replace_at_current_function(
            format!("REPLACE_TYPE_{}", func.name),
            format!("{}", return_type.transpile()),
        );

        Types::Function(FunctionType {
            return_type: Box::from(return_type.clone()),
            parameter_types,
            reference: format!("local::{}", func.name)
        })
    } else {
        Types::Function(FunctionType {
            return_type: Box::from(return_type.clone()),
            parameter_types,
            reference: "".to_string()
        })
    };

    if !func.name.is_empty() {
        let named_function = named_function.unwrap();

        let variable = compiler
            .variable_scope
            .as_ref()
            .borrow_mut()
            .get_variable_mutable(named_function.2, named_function.1);

        variable.unwrap().as_ref().borrow_mut()._type = function_type.clone();
    }

    CompilerResult::Success(function_type)
}
