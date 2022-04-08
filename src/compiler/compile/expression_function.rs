use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::types::{FunctionType, Types};
use std::collections::HashMap;

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
    // Named function ^.^
    if !func.name.is_empty() {
        let mut parameters: Vec<Parameter> = Vec::new();

        for parameter in &func.parameters {
            if parameters
                .iter()
                .find(|&p| p.name == parameter.identifier.value)
                .is_some()
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
        let _type = Types::Function(FunctionType {
            return_type: Box::from(Types::Auto),
            parameter_types: type_parameters,
        });

        let var = compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, func.name),
            _type,
            Modifiers::default(),
        );

        let function = Function {
            name: var.transpile(),
            code: "".to_string(),
            return_type: Types::Auto,
            parameters,
        };

        compiler.new_function(function);
        compiler.add_to_current_function(format!("{} {}", var._type.transpile(), var.transpile()));
    }

    compiler.add_to_current_function(" (".to_string());

    let mut index = 0;
    for parameter in &func.parameters {
        let symbol = compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!(
                "{}{}",
                compiler.location,
                parameter.identifier.value.clone(),
            ),
            parameter._type.clone(),
            Modifiers::default(),
        );

        let mut _type = "Variant".to_string();

        if func.name.is_empty() {
            _type = parameter.get_type()
        }

        // This currently defines every parameter type to be a Variant, we should do compile time
        // checks to ensure type safety.
        compiler.add_to_current_function(format!("{} {}", _type, symbol.transpile()));

        index += 1;

        if func.parameters.len() > 1 && index != func.parameters.len() {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    compiler.add_to_current_function(") ".to_string());

    let result = compiler.compile_block(func.body);

    if !func.name.is_empty() {
        compiler.exit_function();
    }

    result
}
