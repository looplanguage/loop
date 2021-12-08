use crate::compiler::instructions::print_instructions;
use crate::compiler::{build_compiler, CompilerState};
use crate::lexer::build_lexer;
use crate::lib::exception::Exception;
use crate::lib::flags::{FlagTypes, Flags};
use crate::parser::build_parser;
use crate::vm::{build_vm, VMState};
use chrono::Utc;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    line: i32,
    debug: bool,
    compiler_state: Option<CompilerState>,
    vm_state: Option<VMState>,
    benchmark: bool,
    jit: bool,
}

pub fn build_repl(flags: Flags) -> Repl {
    Repl {
        line: 0,
        compiler_state: None,
        vm_state: None,
        debug: flags.contains(FlagTypes::Debug),
        benchmark: flags.contains(FlagTypes::Benchmark),
        jit: flags.contains(FlagTypes::Jit),
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Repl {
    pub fn start(&mut self) -> Option<bool> {
        println!(
            "
██╗░░░░░░█████╗░░█████╗░██████╗░
██║░░░░░██╔══██╗██╔══██╗██╔══██╗
██║░░░░░██║░░██║██║░░██║██████╔╝
██║░░░░░██║░░██║██║░░██║██╔═══╝░
███████╗╚█████╔╝╚█████╔╝██║░░░░░
╚══════╝░╚════╝░░╚════╝░╚═╝░░░░░
        "
        );
        println!("Welcome to Loop v{}", VERSION);

        if self.jit {
            println!(
                "{}You're running Loop in JIT mode. More info: https://looplang.org/docs/internal/jit",
                "WARNING: ".red()
            );
        }

        self.run();

        None
    }

    fn run_code(&mut self, s: String) {
        let l = build_lexer(s.as_str());

        let mut p = build_parser(l);

        let program = p.parse();

        if p.errors.is_empty() {
            let mut compiler = build_compiler(self.compiler_state.as_ref(), self.jit);
            let error = compiler.compile(program);

            if error.is_err() {
                let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
                println!("{}", message.red());
                return;
            }

            self.compiler_state = Some(compiler.get_state());

            if self.debug {
                print_instructions(compiler.scope().instructions.clone());
            }

            let mut vm = build_vm(compiler.get_bytecode(), self.vm_state.as_ref());

            let started = Utc::now();
            let ran = vm.run(self.jit);
            let duration = Utc::now().signed_duration_since(started);

            if ran.is_err() {
                println!(
                    "{}",
                    format!("VirtualMachineException: {}", ran.err().unwrap()).red()
                );
            } else {
                self.vm_state = Some(vm.get_state());

                if self.benchmark {
                    let formatted = duration.to_string().replace("PT", "");
                    println!("Execution Took: {}", formatted);
                }

                println!("{}", ran.ok().unwrap().borrow().inspect().green());
            }
        } else {
            for err in p.errors {
                if let Exception::Parser(str) = err {
                    println!("{}", format!("ParserException: {}", str).red());
                }
            }
        }
    }

    fn run(&mut self) {
        let mut rl = Editor::<()>::new();

        loop {
            self.line += 1;

            let readline = rl.readline(format!("{} => ", self.line).as_str());

            match readline {
                Ok(line) => {
                    if line.as_str() == "exit" {
                        println!("{}", "Exiting the REPL...\n".yellow());
                        break;
                    }
                    rl.add_history_entry(line.as_str());
                    self.run_code(line);
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    let err = format!("{:?}", err);
                    println!("{}", err.red());
                    break;
                }
            }
        }
    }
}
