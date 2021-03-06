use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_string::compile_expression_string;
use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::hashmap::{HashableExpression, Hashmap};
use crate::parser::types::Types;

pub fn compile_expression_hashmap(
    _compiler: &mut Compiler,
    hashmap: Hashmap,
) -> Result<Types, CompilerException> {
    let _length = hashmap.values.len();

    for value in hashmap.values {
        let result = match value.0 {
            HashableExpression::Integer(integer) => compile_expression_integer(_compiler, integer),
            HashableExpression::String(string) => compile_expression_string(_compiler, string),
            HashableExpression::Boolean(boolean) => compile_expression_boolean(_compiler, boolean),
        };

        #[allow(clippy::single_match)]
        match &result {
            Err(_exception) => return result,
            _ => (),
        }

        _compiler.compile_expression(value.1)?;
    }

    Ok(Types::Void)
}
