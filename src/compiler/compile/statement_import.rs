use crate::compiler::Compiler;
use crate::lexer::build_lexer;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::Exception;
use crate::parser::build_parser;
use crate::parser::statement::import::Import;
use std::fs;
use std::path::Path;

pub fn compile_import_statement(
    _compiler: &mut Compiler,
    import: Import,
) -> Option<CompilerException> {
    // Keep hold of last_location
    let last_location = _compiler.location.clone();
    let last_import_location = _compiler.export_name.clone();

    let mut location = Path::new(import.file.as_str());

    let loc = format!("{}\\{}", last_location, import.file);

    if !last_location.is_empty() {
        location = Path::new(loc.as_str());
    }

    let contents = fs::read_to_string(location);

    if contents.is_err() {
        return Some(CompilerException::CanNotReadFile(
            contents.err().unwrap().to_string(),
        ));
    }

    // Set required context for compiling
    _compiler.location = String::from(location.clone().to_str().unwrap());
    _compiler.export_name = import.identifier.clone();

    let contents = contents.unwrap();

    // Lex contents
    let lexer = build_lexer(contents.as_str());

    // Parse & check for errors
    let mut parser = build_parser(lexer);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            if let Exception::Parser(msg) = err {
                println!("ParserException: {}", msg);
            }
        }

        panic!("Parser exceptions occurred!")
    }

    // Compile
    _compiler.compile(program);

    // Set last_location back
    _compiler.location = last_location;
    _compiler.export_name = last_import_location;

    None
}
