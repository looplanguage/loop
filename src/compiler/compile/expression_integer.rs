use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::object::integer;
use crate::object::Object;
use crate::parser::expression::integer::Integer;

pub fn compile_expression_integer(compiler: &mut Compiler, int: Integer) -> Option<String> {
    let ct = compiler.add_constant(Object::Integer(integer::Integer { value: int.value }));
    compiler.emit(OpCode::Constant, vec![ct]);

    None
}
