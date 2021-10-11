use crate::compiler::opcode::OpCode;
use crate::compiler::variable::Scope;
use crate::compiler::Compiler;
use crate::parser::statement::variable::VariableDeclaration;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> Option<String> {
    let find_variable = compiler
        .symbol_table
        .resolve(variable.ident.value.clone());

    if find_variable.is_some() {
        return Some(format!(
            "variable \"{}\" is already declared in this scope",
            find_variable.unwrap().name
        ));
    }

    let id = compiler
        .symbol_table
        .define(variable.ident.value);

    let err = compiler.compile_expression(*variable.value);

    compiler.variable_count += 1;

    compiler.emit(OpCode::SetVar, vec![id.index as u32]);

    if err.is_some() {
        return err;
    }

    None
}
