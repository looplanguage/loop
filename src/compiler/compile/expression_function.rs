use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::object::{function, Object};
use crate::parser::expression::function::Function;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> Option<String> {
    compiler.enter_scope();

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
            .define_variable(second_name, compiler.variable_count);

        compiler.variable_count += 1;
    }

    let err = compiler.compile_block(func.body);
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

    let parameter_len = parameters.len();

    let instructions = compiler.exit_scope();

    let func = function::CompiledFunction {
        instructions,
        parameters,
    };

    let func_id = compiler.add_constant(Object::CompiledFunction(func));

    compiler.emit(OpCode::Function, vec![func_id, parameter_len as u32]);

    None
}
