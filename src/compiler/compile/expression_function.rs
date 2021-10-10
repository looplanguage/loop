use crate::compiler::opcode::OpCode;
use crate::compiler::variable::{Scope, Variable};
use crate::compiler::Compiler;
use crate::object::{function, Object};
use crate::parser::expression::function::Function;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> Option<String> {
    compiler.enter_scope();

    let mut i = 0;
    for parameter in func.parameters {
        let name = parameter.value.clone();
        let find_variable = compiler
            .current_variable_scope
            .find_variable(parameter.value.clone());

        if find_variable.is_some() {
            return Some(format!(
                "parameter name \"{}\" is already a variable name in this scope",
                find_variable.unwrap().name
            ));
        }

        let second_name = name.clone();

        compiler
            .current_variable_scope
            .define_variable(second_name, i);
        i = i + 1;
    }

    let err = compiler.compile_function_block(func.body);
    if err.is_some() {
        return err;
    }

    compiler.remove_last(OpCode::Pop);

    if !compiler.last_is(OpCode::Return) {
        compiler.emit(OpCode::Return, vec![]);
    }

    let mut parameters: Vec<u32> = vec![];

    for variable in &compiler.current_variable_scope.variables {
        parameters.push(variable.index);
    }

    let mut free: Vec<Variable> = vec![];

    for variable in &compiler.current_variable_scope.free {
        free.push(variable.clone());
    }

    println!("{}", free.len());

    let instructions = compiler.exit_scope();

    for free_var in free.clone() {
        if free_var.scope == Scope::Free {
            compiler.emit(OpCode::GetFree, vec![free_var.index]);
        } else {
            compiler.emit(OpCode::GetLocal, vec![free_var.index]);
        }
    }

    let func = function::CompiledFunction {
        instructions,
        parameters,
    };

    let func_id = compiler.add_constant(Object::CompiledFunction(func));

    compiler.emit(OpCode::Function, vec![func_id, free.len() as u32]);

    None
}
