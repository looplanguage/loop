extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::parser::statement::Statement;

mod lexer;
mod parser;

fn main() {
    let l = lexer::build_lexer("var abc = 100 + 100 * 100; 100 + 20; abc; var d = !true");
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            println!("{}", error)
        }

        return;
    }

    println!("Statements ({}): ", program.statements.len());
    for stmt in program.statements {
        match stmt {
            Statement::VariableDeclaration(value) => {
                println!(
                    "Variable declared: {} = {:?}",
                    value.ident.value, value.value
                );
            }
            Statement::Expression(value) => {
                println!("Expression statement: {:?}", value.expression)
            }
        }
    }
}
