use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::Object;
use crate::parser::expression::array;
use crate::parser::expression::array::Array;

pub fn compile_expression_array(compiler: &mut Compiler, arr: Array) -> Option<CompilerException> {
    let array_length = arr.values.len() as u32;

    for value in arr.values {
        compiler.compile_expression(*value.expression);
    }

    compiler.emit(OpCode::Array, vec![array_length]);

    None
}
