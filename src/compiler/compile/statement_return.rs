use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_return_statement(
    _compiler: &mut Compiler,
    rt: ReturnStatement,
) -> Option<CompilerException> {
    if _compiler.scope_index == 0 {
        return Some(CompilerException::ReturnStatementNotAllowedOutsideFunction);
    }

    let err = _compiler.compile_expression(*rt.expression);
    if err.is_some() {
        return err;
    }

    _compiler.emit(OpCode::Return, vec![]);

    None
}
