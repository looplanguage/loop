use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::Function;
use crate::parser::types::Types;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> CompilerResult {
    // Named function ^.^
    if !func.name.is_empty() {
        let var = compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, func.name),
            Types::Function,
            Modifiers::default(),
        );

        compiler.new_function(var.transpile());
        compiler.add_to_current_function(format!("Variant {}", var.transpile()));
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

        compiler.add_to_current_function(format!(
            "{} {}",
            parameter.get_type(),
            symbol.transpile()
        ));

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
