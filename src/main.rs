extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use chrono::Utc;
use colored::Colorize;
use dirs::home_dir;
use std::env;
use std::fs::read_to_string;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::passes::PassManager;

use crate::compiler::instructions::print_instructions;
use crate::lib::config::{load_config, LoadType};
use crate::lib::exception::Exception;
use crate::lib::flags::{FlagTypes, Flags};
use lib::flags;
use lib::repl::build_repl;
use crate::lib::jit::CodeGen;

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
        build_repl(flags).start();
    }
}

fn run_file(file: String, flags: Flags) {
    let content = read_to_string(file);

    /*
    if let Err(e) = content {
        sentry::capture_error(&e);
        return;
    }*/

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

    let mut comp = compiler::build_compiler(None, flags.contains(FlagTypes::Jit));
    let error = comp.compile(program);

    if error.is_err() {
        let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
        println!("{}", message.as_str().red());
        return;
    }

    if flags.contains(FlagTypes::Debug) {
        print_instructions(comp.scope().instructions.clone());
    }

    let mut vm = build_vm(comp.get_bytecode(), None,"MAIN".to_string());

    let started = Utc::now();

    let context = Context::create();
    let module = context.create_module("program");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .ok()
        .ok_or_else(|| "cannot start jit!".to_string()).unwrap();

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let mut codegen = CodeGen {
        context: &context,
        module: &module,
        builder: context.create_builder(),
        execution_engine,
        fpm: &fpm,
        compiled_functions: HashMap::new(),
        parameters: vec![],
        jit_variables: HashMap::new(),
        last_popped: None
    };

    let ran = vm.run(flags.contains(FlagTypes::Jit), codegen.borrow_mut());

    let duration = Utc::now().signed_duration_since(started);

    if ran.is_err() {
        panic!("{}", ran.err().unwrap());
    }

    if flags.contains(FlagTypes::Benchmark) {
        let formatted = duration.to_string().replace("PT", "");
        println!("Execution Took: {}", formatted);
    }

    let last = ran.ok().unwrap();
    println!("{}", last.as_ref().borrow().inspect());
}

fn get_flags() -> flags::Flags {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}
