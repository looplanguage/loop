use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;

pub fn compile_expression_null(compiler: &mut Compiler) -> Option<String> {
    compiler.emit(OpCode::Constant, vec![0]);

    None
}
