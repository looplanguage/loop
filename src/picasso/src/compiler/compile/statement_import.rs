use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::import::Import;
use crate::parser::types::Types;

pub fn compile_import_statement(compiler: &mut Compiler, import: Import) -> CompilerResult {
    compiler.imports.push(import.clone().identifier);

    compiler.add_to_current_function(format!(
        ".LOADLIB {{.CONSTANT CHAR[] \"{}\";}} \"{}\";",
        import.file, import.identifier
    ));
    CompilerResult::Success(Types::Void)
}
