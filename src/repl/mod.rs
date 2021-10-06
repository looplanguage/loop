use crate::compiler::build_compiler;
use crate::compiler::instructions::print_instructions;
use crate::lexer::build_lexer;
use crate::parser::build_parser;
use crate::vm::build_vm;
use std::io::{stdin, stdout, Write};
use colored::Colorize;
use rustyline::Editor;
use rustyline::error::ReadlineError;

pub struct Repl {
    line: i32,
}

pub fn build_repl() -> Repl {
    Repl { line: 0 }
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
            let mut compiler = build_compiler();
            compiler.compile(program);

            print_instructions(compiler.instructions.clone());

            let mut vm = build_vm(compiler.get_bytecode());
            vm.run();

            if vm.last_popped.is_some() {
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

            let readline = rl.readline(format!("{} {} ", self.line, "=>".magenta()).as_str());

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    self.run_code(line);
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    }
}