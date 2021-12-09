extern crate strum;
#[macro_use]
extern crate strum_macros;
use crate::lib::config::{load_config, LoadType};
use crate::lib::util::execute_code;
use dirs::home_dir;
use lib::flags;
use lib::repl::build_repl;
use std::env;
use std::fs::read_to_string;

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

    if let Some(file) = flags.file {
        run_file(file);
    } else {
        build_repl().start();
    }
}

fn run_file(file: String) {
    let content = read_to_string(file);

    let last = execute_code(content.ok().unwrap().as_str(), None, None);
    println!("{}", last.0.ok().unwrap().borrow().inspect());
}

fn get_flags() -> flags::Flags {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}
