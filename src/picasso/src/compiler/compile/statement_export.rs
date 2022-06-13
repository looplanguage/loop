use crate::compiler::{Compiler, CompilerResult};
use crate::compiler::compile::statement_class::compile_class_statement;
use crate::exception::compiler::CompilerException;
use crate::parser::expression;
use crate::parser::expression::Expression::{AssignIndex, Integer};
use crate::parser::expression::{assign_index, identifier, integer};
use crate::parser::expression::function::Parameter;
use crate::parser::expression::identifier::Identifier;
use crate::parser::statement::block::Block;
use crate::parser::statement::class::{Class, ClassField, ClassItem, Method};
use crate::parser::statement::export::Export;
use crate::parser::statement::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_export_statement(_compiler: &mut Compiler, _export: Export) -> CompilerResult {
    // Define a new class and assign it to __export

    let mut methods: Vec<ClassField> = Vec::new();

    let mut index = 0;
    for name in _export.names {
        // Resolve the names and add them to "methods"
        let method = _compiler.resolve_variable(&name);
        
        if let Some(method) = method {
            if let Types::Function(func) = method._type {
                let mut func = func.clone();

                methods.push(ClassField {
                    name,
                    index,
                    item: ClassItem::Lazy(Types::Function(func))
                });

                index += 1;
            }
        }
    }

    methods.push(ClassField {
        name: "constructor".to_string(),
        index: methods.len() as u32,
        item: ClassItem::Method(Method {
            name: "constructor".to_string(),
            return_type: Types::Void,
            arguments: vec![],
            body: Block { statements: methods.iter().map(|item| {
                Statement::Expression(Box::new(
                    Expression {
                        expression: Box::new(AssignIndex(Box::new(assign_index::AssignIndex {
                            left: expression::Expression::Identifier(Identifier {
                                value: "self".to_string()
                            }),
                            index: expression::Expression::Identifier(Identifier {
                                value: item.name.clone()
                            }),
                            value: expression::Expression::Identifier(Identifier {
                                value: item.name.clone()
                            })
                        })))
                    }
                ))
            }).collect() }
        })
    });

    let class = Class {
        name: format!("__export"),
        values: methods,
        inherits: "".to_string()
    };

    let result = compile_class_statement(_compiler, class);

    //let var = _compiler.define_variable("__export".to_string(), )

    CompilerResult::Success(Types::Void)
}
