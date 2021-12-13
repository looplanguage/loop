use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::array::Array;

pub fn compile_expression_array(compiler: &mut Compiler, arr: Array) -> CompilerResult {
    let array_length = arr.values.len() as u32;

    for value in arr.values {
        compiler.compile_expression(*value.expression);
    }

    compiler.emit(OpCode::Array, vec![array_length]);

    CompilerResult::Success
}
