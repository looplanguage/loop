use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::integer;
use crate::lib::object::Object;
use crate::parser::expression::integer::Integer;

pub fn compile_expression_integer(compiler: &mut Compiler, int: Integer) -> CompilerResult {
    let ct = compiler.add_constant(Object::Integer(integer::Integer { value: int.value }));
    compiler.emit(OpCode::Constant, vec![ct]);

    CompilerResult::Success
}
