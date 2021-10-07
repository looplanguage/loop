use crate::compiler::instructions::print_instructions;
use crate::compiler::{build_compiler, CompilerState};
use crate::flags::{FlagTypes, Flags};
use crate::lexer::build_lexer;
use crate::parser::build_parser;
use crate::vm::{build_vm, VMState};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    line: i32,
    debug: bool,
    compiler_state: Option<CompilerState>,
    vm_state: Option<VMState>,
}

pub fn build_repl(flags: Flags) -> Repl {
    Repl {
        line: 0,
        compiler_state: None,
        vm_state: None,
        debug: flags.contains(FlagTypes::Debug),
    }
}

impl Repl {
    pub fn start(&mut self) {
        self.run()
    }

    fn run_code(&mut self, s: String) {
        let l = build_lexer(s.as_str());

        let mut p = build_parser(l);

        let program = p.parse();

        if p.errors.is_empty() {
            let mut compiler = build_compiler(self.compiler_state.as_ref());
            let error = compiler.compile(program);

            if error.is_some() {
                println!("{} {}", "CompilerError:".red(), error.unwrap().red());
                return;
            }

            self.compiler_state = Some(compiler.get_state());

            if self.debug {
                print_instructions(compiler.instructions.clone());
            }

            let mut vm = build_vm(compiler.get_bytecode(), self.vm_state.as_ref());
            vm.run();

            if vm.last_popped.is_some() {
                self.vm_state = Some(vm.get_state());

                println!("{}", vm.last_popped.unwrap().inspect().green());
            }
        } else {
            for err in p.errors {
                println!("{}", err.to_string().red());
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
