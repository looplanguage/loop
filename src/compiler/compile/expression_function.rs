use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
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
        });

        let var = compiler.define_variable(
            format!("{}{}", compiler.location, func.name),
            function_type.clone(),
        );

        named_function = Option::from((var.transpile(), var.name.clone(), var.index));

        let function = Function {
            name: var.transpile(),
            code: "".to_string(),
            return_type: Types::Auto,
            parameters,
        };

        compiler.new_function(function);

        let return_type = {
            if let Types::Function(f) = function_type.clone() {
                f
            } else {
                return CompilerResult::Exception(CompilerException::Unknown);
            }
        };

        compiler.add_to_current_function(format!(
            "{} {}",
            return_type.return_type.transpile(),
            var.transpile()
        ));
    }

    compiler.add_to_current_function(" (".to_string());

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
        );

        let _type = parameter.get_type();

        parameter_types.push(Box::from(parameter._type.clone()));

        // This currently defines every parameter type to be a Variant, we should do compile time
        // checks to ensure type safety.
        compiler.add_to_current_function(format!("{} {}", _type, symbol.transpile()));

        index += 1;

        if func.parameters.len() > 1 && index != func.parameters.len() {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    compiler.add_to_current_function(") ".to_string());

    let result = compiler.compile_block(func.body, func.name.is_empty());

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

    // TODO: Return type is auto for now, but types for parameters is et
    function_type = Types::Function(FunctionType {
        return_type: Box::from(return_type.clone()),
        parameter_types,
    });

    // Set return type of named function, if it exists
    if let Some(named_function) = named_function.clone() {
        let function = compiler.functions.get_mut(&*named_function.0);
        function.unwrap().return_type = return_type.clone();

        compiler.replace_at_current_function(
            format!("Variant {}", named_function.0),
            format!("{} {}", return_type.transpile(), named_function.0),
        );
    }

    if !func.name.is_empty() {
        compiler.exit_function();

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
