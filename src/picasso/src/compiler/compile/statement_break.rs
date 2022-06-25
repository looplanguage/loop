use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::statement::break_statement::BreakStatement;
use crate::parser::types::Types;

pub fn compile_break_statement(
    _compiler: &mut Compiler,
    _br: BreakStatement,
) -> Result<Types, CompilerException> {
    _compiler.add_to_current_function("break".to_string());

    Ok(Types::Void)
}
