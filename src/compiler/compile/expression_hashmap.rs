use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::hashmap::Hashmap;

pub fn compile_expression_hashmap(
    _compiler: &mut Compiler,
    hashmap: Hashmap
) -> Option<CompilerException> {
    None
}