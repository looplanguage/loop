use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::object::integer;
use crate::lib::object::Object;
use crate::parser::expression::integer::Integer;

pub fn compile_expression_integer(compiler: &mut Compiler, int: Integer) -> CompilerResult {
    compiler.add_to_current_function(int.value.to_string());

    CompilerResult::Success
}
