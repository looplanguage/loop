use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::enum_statement::EnumStatement;

pub fn compile_enum_statement(
    compiler: &mut Compiler,
    e: EnumStatement,
) -> Option<CompilerException> {
    None
}
