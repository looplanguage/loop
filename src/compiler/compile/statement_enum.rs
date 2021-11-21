use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::enum_statement::EnumStatement;

pub fn compile_enum_statement(
    _compiler: &mut Compiler,
    rt: EnumStatement,
) -> Option<CompilerException> {

    None
}
