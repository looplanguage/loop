use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::float;
use crate::lib::object::Object;
use crate::parser::expression::float::Float;

pub fn compile_expression_float(compiler: &mut Compiler, flt: Float) -> Option<CompilerException> {
    let ct = compiler.add_constant(Object::Float(float::Float { value: flt.value }));
    compiler.emit(OpCode::Constant, vec![ct]);

    None
}
