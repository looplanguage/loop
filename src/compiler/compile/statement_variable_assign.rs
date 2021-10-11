use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> Option<String> {
    let find_variable = compiler
        .symbol_table
        .resolve(variable.ident.value.clone());

    if find_variable.is_none() {
        return Some(format!(
            "variable \"{}\" is not declared in this scope",
            variable.ident.value
        ));
    }

    let error = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![find_variable.unwrap().index as u32]);

    if error.is_some() {
        return error;
    }

    None
}
