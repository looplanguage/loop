extern crate strum;
#[macro_use]
extern crate strum_macros;

use dirs::home_dir;
use std::env;
use std::fs::read_to_string;

use crate::lib::config::{load_config, LoadType};
use crate::lib::exception::Exception;
use crate::lib::telemetry::enable_telemetry;
use lib::flags;
use lib::repl::build_repl;

use crate::vm::build_vm;

pub mod compiler;
pub mod lexer;
mod lib;
pub mod parser;
mod vm;

fn main() {
    let config = match load_config() {
        LoadType::FirstRun(cfg) => {
            println!("This is your first time running Loop! (Or your config was re-generated)");
            println!("By default we enable telemetry, if you wish to opt out go to: ");
            println!(
                "{}\\.loop\\config.toml",
                home_dir().unwrap().to_str().unwrap()
            );
            println!("If you wish to know more about our privacy policy go to: https://looplang.org/privacy");

            cfg
        }
        LoadType::Normal(cfg) => cfg,
    };

    if config.enable_telemetry {
        enable_telemetry();
    }

    let flags = get_flags();

    if let Some(file) = flags.file {
        run_file(file);
    } else {
        build_repl(flags).start();
    }
}

fn run_file(file: String) {
    let content = read_to_string(file);

    if let Err(e) = content {
        sentry::capture_error(&e);
        return;
    }

    let l = lexer::build_lexer(content.unwrap().as_str());
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            if let Exception::Parser(msg) = err {
                println!("ParserException: {}", msg);
            }
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
                sentry::capture_message(err.clone().unwrap().as_str(), sentry::Level::Info);
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
