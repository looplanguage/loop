extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::repl::build_repl;
use std::env;

pub mod compiler;
mod flags;
pub mod lexer;
pub mod object;
pub mod parser;
mod repl;
mod vm;

fn main() {
    build_repl(get_flags()).start();
}

fn get_flags() -> flags::Flags {
    let args: Vec<String> = env::args().collect();
    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}
