use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::{Function, Parameter};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::statement::class::{Class, ClassItem};
use crate::parser::types::{BaseTypes, ClassItemType, Compound, FunctionType, Types};

pub fn compile_class_statement(compiler: &mut Compiler, class: Class) -> CompilerResult {
    let mut items: Vec<ClassItemType> = Vec::new();

    compiler.add_to_current_function(format!(".COMPOUND \"{}\" {{ ", class.name));

    let var = compiler.define_variable(class.name.clone(), Types::Auto, 0);

    let inherits = compiler.variable_scope.borrow_mut().resolve(class.inherits);

    if let Some(inherits) = inherits {
        if let Types::Compound(inherits) = inherits._type {
            let inherited_handles: Vec<_> = inherits.1.iter().collect();

            for inherited_handle in inherited_handles {
                items.push(inherited_handle.clone());
            }
        }
    }

    for class_item in class.values {
        let class_item = class_item.clone();
        let name = class_item.name.clone();
        let index = class_item.index;
        let mut replace = None;

        if let Some(it) = items.iter_mut().find(|item| item.name == name) {
            replace = Some(it);
        }

        match class_item.item {
            ClassItem::Property(property) => {
                compiler.drier();
                let node = compiler.compile_expression(*property.expression.clone());
                compiler.undrier();

                if let CompilerResult::Success(_type) = node {
                    compiler.add_to_current_function(format!("{};", _type.transpile()));

                    let mut new_item = ClassItemType {
                        name,
                        index,
                        class_item_type: _type,
                        value: *property.expression.clone(),
                    };

                    if let Some(replace) = replace {
                        new_item.index = replace.index;
                        *replace = new_item
                    } else {
                        items.push(new_item)
                    }
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

                let mut new_item = ClassItemType {
                    name: name.clone(),
                    index,
                    class_item_type: Types::Function(FunctionType {
                        return_type: Box::new(method.return_type.clone()),
                        parameter_types: method
                            .arguments
                            .clone()
                            .into_iter()
                            .map(|v| v._type)
                            .collect(),
                        reference: "".to_string(),
                    }),
                    value: Expression::Function(Function {
                        name: name.clone(),
                        parameters: method.arguments.clone(),
                        body: method.body.clone(),
                        predefined_type: Some(method.return_type.clone()),
                    }),
                };

                if let Some(replace) = replace {
                    new_item.index = replace.index;
                    *replace = new_item
                } else {
                    items.push(new_item)
                }

                compiler.add_to_current_function(format!("{};", method.return_type.transpile()));
            }
            ClassItem::Lazy(lazy) => {
                let value = Expression::Integer(Integer { value: 0 });

                let mut new_item = ClassItemType {
                    name,
                    index,
                    class_item_type: lazy.clone(),
                    value,
                };

                if let Some(replace) = replace {
                    new_item.index = replace.index;
                    *replace = new_item
                } else {
                    items.push(new_item)
                }

                compiler.add_to_current_function(format!("{};", lazy.transpile()))
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
