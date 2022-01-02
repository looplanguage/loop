use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
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
    let result = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => result,
        _ => CompilerResult::Success,
    }
}
