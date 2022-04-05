use crate::compiler::{Compiler, CompilerResult};
use crate::lexer::build_lexer;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::Exception;
use crate::parser::build_parser;
use crate::parser::statement::import::Import;
use std::fs;
use std::path::Path;

pub fn compile_import_statement(_compiler: &mut Compiler, import: Import) -> CompilerResult {
    // Keep hold of last_location
    let before_last_location = _compiler.prev_location.clone();
    let last_location = _compiler.location.clone();
    let last_import_location = _compiler.export_name.clone();

    let mut location = Path::new(import.file.as_str());

    let loc = Path::new(last_location.as_str()).join(location);

    if !last_location.is_empty() {
        location = loc.as_path();
    }

    let contents = fs::read_to_string(location);

    if contents.is_err() {
        return CompilerResult::Exception(CompilerException::CanNotReadFile(
            contents.err().unwrap().to_string(),
        ));
    }

    // Set required context for compiling
    _compiler.prev_location = last_location.clone();
    _compiler.location = location
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('.', "_")
        .replace('/', "_");
    _compiler.export_name = import.identifier.clone();

    let contents = contents.unwrap();

    // Lex contents
    let lexer = build_lexer(contents.as_str());

    // Parse & check for errors
    let mut parser = build_parser(lexer);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            if let Exception::Syntax(msg) = err {
                println!("ParserException: {}", msg);
            }
        }

        panic!("Parser exceptions occurred!")
    }

    // Compile
    let err = _compiler.compile(program);

    if err.is_err() {
        return CompilerResult::Exception(err.err().unwrap());
    }

    // Set last_location back
    _compiler.prev_location = before_last_location;
    _compiler.location = last_location;
    _compiler.export_name = last_import_location;

    CompilerResult::Success
}
