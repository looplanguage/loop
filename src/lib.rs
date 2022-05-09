use std::process::exit;

mod compiler;
pub mod exception;
mod lexer;
mod parser;

pub fn compile(str: &str) -> String {
    let lexer = lexer::build_lexer(str);
    let mut parser = parser::build_parser(lexer);

    let program = parser.parse();
    let mut compiler = compiler::Compiler::default();

    let compiled = compiler.compile(program);

    if compiled.is_err() {
        println!("Picasso: {:?}", compiled.err().unwrap());
        exit(1);
    }

    compiled.unwrap().get_arc()
}
