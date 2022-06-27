use crate::compiler::compile::expression_function::compile_expression_function;
use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::function::{Function, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::statement::class::{ClassItem, Method};
use crate::parser::statement::extends::ExtendStatement;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_extend_statement(
    compiler: &mut Compiler,
    class: ExtendStatement,
) -> Result<Types, CompilerException> {
    // Just for the basetypes
    let raw_type = match &*class.identifier.value {
        "int" => Some(Types::Basic(BaseTypes::Integer)),
        "string" => Some(Types::Basic(BaseTypes::String)),
        "bool" => Some(Types::Basic(BaseTypes::Boolean)),
        "float" => Some(Types::Basic(BaseTypes::Float)),
        _ => None,
    };

    if raw_type.is_none() {
        return Err(CompilerException::Unknown);
    }

    if let Some(raw_type) = raw_type {
        // Create if does not exist
        let mut extensions: Option<&mut Vec<Method>> =
            compiler.extensions.get_mut(&raw_type.transpile());

        if extensions.is_none() {
            compiler.extensions.insert(raw_type.transpile(), vec![]);

            extensions = compiler.extensions.get_mut(&raw_type.transpile());
        }

        let unwrapped = extensions.unwrap();

        for item in &class.items {
            if let ClassItem::Method(method) = item.item.clone() {
                unwrapped.push(method.clone());
            }
        }

        for item in &class.items {
            if let ClassItem::Method(method) = item.item.clone() {
                let mut params = method.arguments.clone();
                params.insert(
                    0,
                    Parameter {
                        identifier: Identifier {
                            value: "self".to_string(),
                        },
                        _type: raw_type.clone(),
                    },
                );

                compile_expression_function(
                    compiler,
                    Function {
                        name: format!("{}_{}", raw_type.transpile(), method.name),
                        parameters: params,
                        body: method.body.clone(),
                        predefined_type: None,
                        public: false,
                    },
                )?;
            }
        }
    }

    Ok(Types::Void)
}
