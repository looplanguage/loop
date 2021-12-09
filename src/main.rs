extern crate strum;
#[macro_use]
extern crate strum_macros;

use chrono::Utc;
use colored::Colorize;
use dirs::home_dir;
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::OptimizationLevel;
use std::borrow::BorrowMut;
use std::env;
use std::fs::read_to_string;

use crate::compiler::instructions::print_instructions;
use crate::lib::config::{load_config, LoadType};
use crate::lib::exception::Exception;
use crate::lib::flags::{FlagTypes, Flags};
use crate::lib::jit::CodeGen;
use lib::flags;
use lib::repl::build_repl;
use crate::lib::util::execute_code;

use crate::vm::build_vm;

pub mod compiler;
pub mod lexer;
mod lib;
pub mod parser;
mod vm;

fn main() {
    let _config = match load_config() {
        LoadType::FirstRun(cfg) => {
            println!("This is your first time running Loop! (Or your config was re-generated)");
            println!("Configuration file is at: ");
            println!(
                "{}\\.loop\\config.toml",
                home_dir().unwrap().to_str().unwrap()
            );

            cfg
        }

        LoadType::Normal(cfg) => cfg,
    };

    let flags = get_flags();

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "version" {
        println!("{}", env!("CARGO_PKG_VERSION"));

        return;
    }

    if let Some(file) = flags.file.clone() {
        run_file(file, flags);
    } else {
        build_repl().start();
    }
}

fn run_file(file: String, flags: Flags) {
    let content = read_to_string(file);

    let last = execute_code(content.ok().unwrap().as_str(), None);
    println!("{}", last.0.ok().unwrap().borrow().inspect());
}

fn get_flags() -> flags::Flags {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}
