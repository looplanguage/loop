extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::compiler::instructions::print_instructions;
use crate::parser::statement::Statement;

pub mod lexer;
pub mod parser;
pub mod compiler;
pub mod object;

fn main() {
    let l = lexer::build_lexer(
        "
        1; 2
        ",
    );
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            println!("{}", error)
        }

        return;
    }

    println!("Statements ({}): ", program.statements.len());
    let mut comp = compiler::build_compiler();
    comp.compile(program);

    println!("Instructions ({}): ", comp.instructions.len());
    print_instructions(comp.instructions);
}
