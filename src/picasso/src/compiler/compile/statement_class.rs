use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::Expression;
use crate::parser::statement::class::Class;
use crate::parser::statement::import::Import;
use crate::parser::types::Types;
use std::collections::HashMap;
use std::fmt::format;

pub fn compile_class_statement(compiler: &mut Compiler, class: Class) -> CompilerResult {
    let mut items: HashMap<String, (u32, (Types, Expression))> = HashMap::new();

    compiler.add_to_current_function(format!(".COMPOUND \"{}\" {{ ", class.name));

    let var = compiler.define_variable(
        class.name.clone(),
        Types::Auto,
        0,
    );

    for class_item in class.values.iter().enumerate() {
        compiler.dry = true;
        let node = compiler.compile_expression(*class_item.1 .1.clone().expression);
        compiler.dry = false;

        if let CompilerResult::Success(t) = node {
            compiler.add_to_current_function(format!("{};", t.transpile()));

            items.insert(
                class_item.1 .0.clone(),
                (
                    class_item.0 as u32,
                    (t, *class_item.1 .1.clone().expression.clone()),
                ),
            );
        } else {
            return node;
        }
    }

    compiler.add_to_current_function("};".to_string());

    let var = compiler.variable_scope.borrow_mut().get_variable_mutable(var.index, var.name.clone());

    var.unwrap().as_ref().borrow_mut()._type = Types::Compound(class.name.clone(), Box::new(items));

    CompilerResult::Success(Types::Void)
}
