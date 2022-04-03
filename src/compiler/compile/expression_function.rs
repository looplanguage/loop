use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::Function;
use crate::parser::expression::Expression;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> CompilerResult {
    // Named function ^.^
    if !func.name.is_empty() {
        let var = compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, func.name),
            Expression::Function(func.clone()),
        );

        compiler.new_function(var.transpile());
        compiler.add_to_current_function(format!("auto {}", var.transpile()));
    }

    compiler.add_to_current_function(" (".to_string());

    let mut index = 0;
    for parameter in &func.parameters {
        let symbol = *compiler
            .symbol_table
            .borrow_mut()
            .define(parameter.identifier.value.as_str(), 0);

        compiler.add_to_current_function(format!(
            "{} local_{}",
            parameter.get_type(),
            symbol.index
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
