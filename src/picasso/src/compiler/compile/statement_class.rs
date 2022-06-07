use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::{Function, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::class::{Class, ClassItem};
use crate::parser::statement::Statement;
use crate::parser::types::{BaseTypes, ClassItemType, Compound, Types};
use std::collections::HashMap;

pub fn compile_class_statement(compiler: &mut Compiler, class: Class) -> CompilerResult {
    let mut items: HashMap<String, ClassItemType> = HashMap::new();

    compiler.add_to_current_function(format!(".COMPOUND \"{}\" {{ ", class.name.clone()));

    let var = compiler.define_variable(class.name.clone(), Types::Auto, 0);

    for class_item in class.values.iter().enumerate() {
        let name = class_item.1 .0.clone();
        let index = class_item.0 as u32;

        match class_item.1 .1 {
            ClassItem::Property(property) => {
                compiler.drier();
                let node = compiler.compile_expression(*property.expression.clone());
                compiler.undrier();

                if let CompilerResult::Success(_type) = node {
                    compiler.add_to_current_function(format!("{};", _type.transpile()));

                    items.insert(
                        name,
                        ClassItemType {
                            index,
                            class_item_type: _type,
                            value: *property.expression.clone(),
                        },
                    )
                } else {
                    return node;
                };
            }
            ClassItem::Method(method) => {
                // For methods we won't compile the expression yet, as we are not sure yet
                // what the self reference fully contains. Ok this explanation really sucks
                // and I'm not sure yet how to make it more clear. So, learn PL design please :)

                let mut arguments = method.arguments.clone();

                arguments.insert(
                    0,
                    Parameter {
                        identifier: Identifier {
                            value: "self".to_string(),
                        },
                        _type: Types::Basic(BaseTypes::UserDefined(class.name.clone())),
                    },
                );

                items.insert(
                    name.clone(),
                    ClassItemType {
                        index,
                        class_item_type: method.return_type.clone(),
                        value: Expression::Function(Function {
                            name: name.clone(),
                            parameters: method.arguments.clone(),
                            body: method.body.clone(),
                            predefined_type: Some(method.return_type.clone()),
                        }),
                    },
                );

                compiler.add_to_current_function(format!("{};", method.return_type.transpile()));
            }
        }
    }

    compiler.add_to_current_function("};".to_string());

    let var = compiler
        .variable_scope
        .borrow_mut()
        .get_variable_mutable(var.index, var.name);

    var.unwrap().as_ref().borrow_mut()._type =
        Types::Compound(Compound(class.name, Box::new(items)));

    CompilerResult::Success(Types::Void)
}
