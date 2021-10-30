use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::import::Import;

pub fn compile_import_statement(
    _compiler: &mut Compiler,
    import: Import,
) -> Option<CompilerException> {
    None
}
