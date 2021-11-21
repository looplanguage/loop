use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_enum_statement(
    _compiler: &mut Compiler,
    rt: ReturnStatement,
) -> Option<CompilerException> {

    None
}
