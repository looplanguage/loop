extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::compiler::instructions::print_instructions;
use crate::repl::build_repl;

pub mod compiler;
pub mod lexer;
pub mod object;
pub mod parser;
mod repl;
mod vm;

fn main() {
    build_repl().start();
}
