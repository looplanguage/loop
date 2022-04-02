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

    _compiler.add_to_current_function("return ".to_string());

    let result = _compiler.compile_expression(*rt.expression);

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    CompilerResult::Success
}
