use std::borrow::Borrow;
use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::export::Export;
use crate::parser::types::Types;

pub fn compile_export_statement(_compiler: &mut Compiler, _export: Export) -> CompilerResult {
    // Define a new class and assign it to __export
    CompilerResult::Success(Types::Void)
}
