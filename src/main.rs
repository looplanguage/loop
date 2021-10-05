extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::compiler::instructions::print_instructions;

pub mod compiler;
pub mod lexer;
pub mod object;
pub mod parser;
mod vm;

fn main() {
    let l = lexer::build_lexer(
        "
        300 + 50
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

    let mut comp = compiler::build_compiler();
    comp.compile(program);

    print_instructions(comp.instructions.clone());

    let mut vm = vm::build_vm(comp.get_bytecode());
    vm.run();

    println!("{}", vm.last_popped.unwrap().inspect())
}
