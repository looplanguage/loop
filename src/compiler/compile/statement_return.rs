use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_return_statement(_compiler: &mut Compiler, rt: ReturnStatement) -> CompilerResult {
    if _compiler.scope_index == 0 {
        return CompilerResult::Exception(
            CompilerException::ReturnStatementNotAllowedOutsideFunction,
        );
    }

    let result = _compiler.compile_expression(*rt.expression);

    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    _compiler.emit(OpCode::Return, vec![]);

    CompilerResult::Success
}
