use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::statement::import::Import;
use crate::parser::types::{Library, Types};

pub fn compile_import_statement(_compiler: &mut Compiler, _import: Import) -> CompilerResult {
    // TODO: Change after demo lmao
    // TODO: Change after demo lmao
    // TODO: Change after demo lmao
    _compiler.define_variable(_import.identifier.clone(), Types::Library(
        Library {
            methods: vec!["create".to_string(), "run".to_string()]
        }
    ));

    _compiler.add_to_current_function(format!(".LOADLIB {{.CONSTANT CHAR[] \"{}\";}} \"{}\";", _import.file, _import.identifier));
    CompilerResult::Success(Types::Void)
}
