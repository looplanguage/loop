use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::class::Class;
use crate::parser::statement::import::Import;
use crate::parser::types::Types;
use std::collections::HashMap;
use std::fmt::format;
use crate::parser::expression::Expression;

pub fn compile_class_statement(compiler: &mut Compiler, class: Class) -> CompilerResult {
    let mut items: HashMap<String, (u32, (Types, Expression))> = HashMap::new();

    compiler.add_to_current_function(format!(".COMPOUND \"{}\" {{ ", class.name));

    for class_item in class.values.iter().enumerate() {
        compiler.dry = true;
        let node = compiler.compile_expression(*class_item.1.1.clone().expression);
        compiler.dry = false;

        if let CompilerResult::Success(t) = node {
            compiler.add_to_current_function(format!("{};", t.transpile()));

            items.insert(class_item.1.0.clone(), (class_item.0 as u32, (t, *class_item.1.1.clone().expression.clone())));
        }
    }

    compiler.add_to_current_function("};".to_string());

    compiler.define_variable(
        class.name.clone(),
        Types::Compound(class.name.clone(), Box::new(items)),
        0,
    );

    CompilerResult::Success(Types::Void)
}
