use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;

pub fn compile_expression_null(compiler: &mut Compiler) -> Option<CompilerException> {
    compiler.emit(OpCode::Constant, vec![0]);

    None
}
