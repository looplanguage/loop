use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::variable::VariableDeclaration;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> CompilerResult {
    let var = compiler.variable_scope.borrow_mut().define(
        compiler.variable_count,
        format!("{}{}", compiler.location, variable.ident.value),
        *variable.value.clone(),
    );

    compiler.variable_count += 1;
    let err = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    if err.is_some() {
        return CompilerResult::Exception(err.unwrap());
    }

    CompilerResult::Success
}
