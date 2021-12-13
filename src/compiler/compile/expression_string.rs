use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::object::string::LoopString;
use crate::lib::object::Object;
use crate::parser::expression;

pub fn compile_expression_string(
    compiler: &mut Compiler,
    string: expression::string::LoopString,
) -> CompilerResult {
    let ct = compiler.add_constant(Object::String(LoopString {
        value: string.value,
    }));
    compiler.emit(OpCode::Constant, vec![ct]);

    CompilerResult::Success
}
