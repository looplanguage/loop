use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};

pub fn compile_expression_null(compiler: &mut Compiler) -> CompilerResult {
    compiler.emit(OpCode::Constant, vec![0]);
    CompilerResult::Success
}
