use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::Compiler;
use crate::compiler::opcode::OpCode;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::hashmap::{HashableExpression, Hashmap};

pub fn compile_expression_hashmap(
    _compiler: &mut Compiler,
    hashmap: Hashmap
) -> Option<CompilerException> {
    let length = hashmap.values.len();

    for value in hashmap.values {
        match value.0 {
            HashableExpression::Integer(integer) => compile_expression_integer(_compiler, integer),
        };

        _compiler.compile_expression(value.1);
    }

    _compiler.emit(OpCode::Hashmap, vec![length as u32]);

    None
}