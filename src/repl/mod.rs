use std::io::{BufRead, stdin, stdout, Write};
use crate::compiler::build_compiler;
use crate::compiler::instructions::print_instructions;
use crate::lexer::build_lexer;
use crate::parser::build_parser;
use crate::vm::build_vm;

pub struct Repl {
    line: i32
}

pub fn build_repl() -> Repl {
    Repl {
        line: 0
    }
}

impl Repl {
    pub fn start(&mut self) {
        self.run()
    }

    fn run(&mut self) {
        self.line += 1;

        let mut s=String::new();
        print!("{} > ", self.line);
        let _ = stdout().flush();
        stdin().read_line(&mut s);

        let l = build_lexer(s.as_str());
        let mut p = build_parser(l);

        if p.errors.is_empty() {
            let program = p.parse();
            let mut compiler = build_compiler();
            compiler.compile(program);

            print_instructions(compiler.instructions.clone());

            let mut vm = build_vm(compiler.get_bytecode());
            vm.run();

            if vm.last_popped.is_some() {
                println!("{}", vm.last_popped.unwrap().inspect());
            }
        } else {
            for err in p.errors {
                println!("{}", err);
            }
        }

        self.run();
    }
}

/*
            let l = build_lexer(line.unwrap().as_str());
            let mut p = build_parser(l);
            let program = p.parse();
            let mut compiler = build_compiler();
            compiler.compile(program);
            let vm = build_vm(compiler.get_bytecode());
*/