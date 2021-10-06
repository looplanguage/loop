extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::repl::build_repl;
use std::env;

pub mod compiler;
pub mod lexer;
pub mod object;
pub mod parser;
mod repl;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = parse_flags(args);

    build_repl(flags).start();
}

#[derive(PartialEq)]
pub enum Flags {
    None,
    Debug,
}

fn get_flag(string: &str) -> Flags {
    match string {
        "--debug" | "-d" => Flags::Debug,
        &_ => Flags::None,
    }
}

fn parse_flags(args: Vec<String>) -> Vec<Flags> {
    let mut flags: Vec<Flags> = vec![];

    for arg in args {
        let flag = get_flag(arg.as_str());

        if flag != Flags::None {
            flags.push(flag)
        }
    }

    flags
}
