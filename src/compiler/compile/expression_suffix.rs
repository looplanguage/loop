use crate::compiler::Compiler;
use crate::compiler::opcode::OpCode;
use crate::parser::expression::suffix::Suffix;

pub fn compile_expression_suffix(_compiler: &mut Compiler, _suffix: Suffix) -> Option<String> {
    _compiler.compile_expression(_suffix.left);
    _compiler.compile_expression(_suffix.right);

    _compiler.emit(OpCode::Add, vec![]);

    None
}
