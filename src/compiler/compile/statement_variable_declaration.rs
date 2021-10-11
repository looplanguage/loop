use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::statement::variable::VariableDeclaration;
use std::env::var;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> Option<String> {
    let symbol = *compiler
        .symbol_table
        .borrow_mut()
        .define(&*variable.ident.value, compiler.variable_count);

    compiler.variable_count += 1;
    let err = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![symbol.index as u32]);

    if err.is_some() {
        return err;
    }

    None
}
