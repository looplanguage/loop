use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::object::float;
use crate::object::Object;
use crate::parser::expression::float::Float;

pub fn compile_expression_float(compiler: &mut Compiler, flt: Float) -> Option<String> {
    let ct = compiler.add_constant(Object::Float(float::Float { value: flt.value }));
    compiler.emit(OpCode::Constant, vec![ct]);

    None
}
