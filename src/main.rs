extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::repl::build_repl;
use crate::vm::build_vm;
use std::env;
use std::fs::read_to_string;

pub mod compiler;
mod flags;
pub mod lexer;
pub mod object;
pub mod parser;
mod repl;
mod vm;

#[cfg(debug_assertions)]
fn get_build() -> String {
    "development".to_string()
}

#[cfg(not(debug_assertions))]
fn get_build() -> String {
    "release".to_string()
}

fn main() {
    let _guard = sentry::init(("https://d071f32c72f44690a1a7f9821cd15ace@o1037493.ingest.sentry.io/6005454", sentry::ClientOptions {
        release: sentry::release_name!(),
        environment: Some(get_build().into()),
        ..Default::default()
    }));

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
        sentry::with_scope(|scope| {
            scope.set_tag("exception.type", "vm");
        }, || {
            sentry::capture_message(format!("{}", err.clone().unwrap()).as_str(), sentry::Level::Info);
        });

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
