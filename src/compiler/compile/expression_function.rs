use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::config::CONFIG;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::function;
use crate::lib::object::Object::CompiledFunction;
use crate::parser::expression::Expression;
use crate::parser::expression::function::Function;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> CompilerResult {
    // Named function ^.^
    if func.name.len() > 0 {
        let var = compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, func.name.clone()),
            Expression::Function(func.clone()),
        );

        compiler.new_function(var.transpile());
        compiler.add_to_current_function(format!("auto {}", var.transpile()));
    }

    let num_params = func.parameters.len() as u32;

    compiler.enter_scope();

    compiler.add_to_current_function(" (".to_string());

    let mut index = 0;
    for parameter in &func.parameters {

        let symbol = compiler
            .symbol_table
            .borrow_mut()
            .define(parameter.identifier.value.as_str(), 0).clone();

        compiler.add_to_current_function(format!("{} local_{}", parameter.get_type(), symbol.index));

        if func.parameters.len() > 1 && index != func.parameters.len() {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    compiler.add_to_current_function(") ".to_string());

    compiler.compile_block(func.body);

    if func.name.len() > 0 {
        compiler.exit_function();
    }

    CompilerResult::Success
}
