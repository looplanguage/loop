use crate::compiler::CompilerState;
use std::path::Path;
use std::process::{exit, ExitCode};

pub mod compiler;
pub mod exception;
mod lexer;
mod parser;

pub fn compile_with_state(str: &str, state: CompilerState) -> (String, CompilerState) {
    let lexer = lexer::build_lexer(str);
    let mut parser = parser::build_parser(lexer, "");

    let program = parser.parse().unwrap();
    let mut compiler = compiler::Compiler::default_with_state(state);

    let compiled = compiler.compile(program);

    if compiled.is_err() {
        println!("Picasso: {:?}", compiled.err().unwrap());
        exit(1);
    }

    (compiled.unwrap().get_arc(), compiler.get_compiler_state())
}

pub fn compile(
    str: &str,
    file_location: Option<&str>,
) -> Result<(String, CompilerState), ExitCode> {
    let lexer = lexer::build_lexer(str);
    let mut parser = parser::build_parser(lexer, file_location.unwrap_or(""));

    let program = parser.parse();

    if program.is_err() {
        return Err(ExitCode::FAILURE);
    }

    let program = program.unwrap();

    let mut compiler = compiler::Compiler::default();
    if let Some(file) = file_location {
        let path = Path::new(file);
        if path.extension().is_some() {
            compiler.base_location = path.parent().unwrap().to_str().unwrap().to_string()
        } else {
            compiler.base_location = file.to_string();
        }
    }

    let compiled = compiler.compile(program);

    if compiled.is_err() {
        println!("Picasso: {:?}", compiled.err().unwrap());
        exit(1);
    }

    Ok((compiled.unwrap().get_arc(), compiler.get_compiler_state()))
}
