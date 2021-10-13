extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::env;
use std::fs::read_to_string;

use lib::flags;
use lib::repl::build_repl;

use crate::vm::build_vm;

pub mod compiler;
pub mod lexer;
mod lib;
pub mod parser;
mod vm;

fn main() {
    let flags = get_flags();

    if let Some(file) = flags.file {
        run_file(file);
    } else {
        build_repl(flags).start();
    }
}

fn run_file(file: String) {
    let content = read_to_string(file);

    if content.is_err() {
        sentry::capture_error(&content.unwrap_err());
        return;
    }

    let l = lexer::build_lexer(content.unwrap().as_str());
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            println!("ParserException: {}", err);
        }

        panic!("Parser exceptions occurred!")
    }

    let mut comp = compiler::build_compiler(None);
    let err = comp.compile(program);

    if err.is_some() {
        panic!("{}", err.unwrap());
    }

    let mut vm = build_vm(comp.get_bytecode(), None);
    let err = vm.run();

    if err.is_some() {
        sentry::with_scope(
            |scope| {
                scope.set_tag("exception.type", "vm");
            },
            || {
                sentry::capture_message(
                    format!("{}", err.clone().unwrap()).as_str(),
                    sentry::Level::Info,
                );
            },
        );

        panic!("{}", err.unwrap());
    }

    let last_popped = vm.last_popped;

    if let Some(last) = last_popped {
        println!("{}", last.inspect());
    } else {
        println!("VMException: VM did not return anything")
    }
}

fn get_flags() -> flags::Flags {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}
