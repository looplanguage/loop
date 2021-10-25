use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::Object;
use crate::parser::expression::array;
use crate::parser::expression::array::Array;

pub fn compile_expression_integer(
    compiler: &mut Compiler,
    arr: Array,
) -> Option<CompilerException> {
    let ct = compiler.add_constant(Object::Array(array::Array { values: arr.values }));
    compiler.emit(OpCode::Constant, vec![ct]);

    None
}