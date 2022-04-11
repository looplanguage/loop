use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::break_statement::BreakStatement;
use crate::parser::types::Types;

pub fn compile_break_statement(_compiler: &mut Compiler, _br: BreakStatement) -> CompilerResult {
    _compiler.add_to_current_function("break".to_string());

    CompilerResult::Success(Types::Void)
}
