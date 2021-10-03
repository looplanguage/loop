extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::parser::statement::Statement;

mod lexer;
mod parser;

fn main() {
    let l = lexer::build_lexer("var test = 1; var testtwo = 2;");
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    for stmt in program.statements {
        match stmt {
            Statement::VariableDeclaration(value) => {
                println!("Variable declared: {} = {:?}", value.ident.value, value.value);
            }
        }
    }
}
