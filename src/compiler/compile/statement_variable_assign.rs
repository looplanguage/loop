use crate::compiler::Compiler;
use crate::compiler::opcode::OpCode;
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(compiler: &mut Compiler, variable: VariableAssign) -> Option<String> {
    let find_variable = compiler
        .current_variable_scope
        .find_variable(variable.ident.value.clone());

    if find_variable.is_none() {
        return Some(format!(
            "variable \"{}\" is not declared in this scope",
            variable.ident.value
        ));
    }

    let error = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![find_variable.unwrap().index]);

    if error.is_some() {
        return error
    }

    None
}