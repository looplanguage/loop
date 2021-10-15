extern crate strum;
#[macro_use]
extern crate strum_macros;

use chrono::Utc;
use colored::Colorize;
use dirs::home_dir;
use std::env;
use std::fs::read_to_string;

use crate::lib::config::{load_config, LoadType};
use crate::lib::exception::Exception;
use crate::lib::flags::{FlagTypes, Flags};
use lib::flags;
use lib::repl::build_repl;

use crate::vm::build_vm;

pub mod compiler;
pub mod lexer;
mod lib;
pub mod parser;
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

    let _guard = sentry::init((
        "https://d071f32c72f44690a1a7f9821cd15ace@o1037493.ingest.sentry.io/6005454",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(get_build().into()),
            ..Default::default()
        },
    ));

    if !config.enable_telemetry {
        _guard.close(None);
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
    let error = comp.compile(program);

    if error.is_err() {
        let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
        println!("{}", message.as_str().red());
        return;
    }

    let mut vm = build_vm(comp.get_bytecode(), None);
    let ran = vm.run();

    let started = Utc::now();

    let duration = Utc::now().signed_duration_since(started);

    if ran.is_err() {
        sentry::with_scope(
            |scope| {
                scope.set_tag("exception.type", "vm");
            },
            || {
                sentry::capture_message(ran.err().clone().unwrap().as_str(), sentry::Level::Info);
            },
        );

        panic!("{}", ran.err().unwrap());
    }

    if flags.contains(FlagTypes::Benchmark) {
        let formatted = duration.to_string().replace("PT", "");
        println!("Execution Took: {}", formatted);
    }

    if let Some(last) = ran.ok() {
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
