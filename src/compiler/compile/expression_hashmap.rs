use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_string::compile_expression_string;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::hashmap::{HashableExpressionHashmap, Hashmap};

pub fn compile_expression_hashmap(
    _compiler: &mut Compiler,
    hashmap: Hashmap,
) -> Option<CompilerException> {
    let length = hashmap.values.len();

    for value in hashmap.values {
        match value.0 {
            HashableExpressionHashmap::Integer(integer) => {
                compile_expression_integer(_compiler, integer)
            }
            HashableExpressionHashmap::String(string) => {
                compile_expression_string(_compiler, string)
            }
            HashableExpressionHashmap::Boolean(boolean) => {
                compile_expression_boolean(_compiler, boolean)
            }
        };

        _compiler.compile_expression(value.1);
    }

    _compiler.emit(OpCode::Hashmap, vec![length as u32]);

    None
}
