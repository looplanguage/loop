use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::statement::import::Import;
use crate::parser::types::Types;

pub fn compile_import_statement(_compiler: &mut Compiler, _import: Import) -> CompilerResult {
    _compiler.add_to_current_function(format!(".LOADLIB {{.CONSTANT CHAR[] \"{}\";}} \"{}\";", _import.file, _import.identifier));
    CompilerResult::Success(Types::Void)
}
