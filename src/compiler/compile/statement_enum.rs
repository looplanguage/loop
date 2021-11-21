use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::enum_statement::EnumStatement;

pub fn compile_enum_statement(
    compiler: &mut Compiler,
    e: EnumStatement,
) -> Option<CompilerException> {
    let enum_length = e.identifiers.len() as u32;

    for ident in e.identifiers {
        compiler.compile_expression_identifier(compiler, ident);
    }

    compiler.emit(OpCode::Enum, vec![enum_length]);

    None
}
