use crate::compiler::instructions::print_instructions;
use crate::compiler::{build_compiler, CompilerState};
use crate::lexer::build_lexer;
use crate::lib::config::CONFIG;
use crate::lib::exception::Exception;
use crate::lib::jit::CodeGen;
use crate::parser::build_parser;
use crate::vm::{build_vm, VMState};
use chrono::Utc;
use colored::Colorize;
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::OptimizationLevel;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    line: i32,
    compiler_state: Option<CompilerState>,
    vm_state: Option<VMState>,
}

pub fn build_repl() -> Repl {
    Repl {
        line: 0,
        compiler_state: None,
        vm_state: None,
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Repl {
    pub fn start(&mut self) -> Option<bool> {
        println!(
            "
  _
 | |       ___     ___    _ __
 | |      / _ \\   / _ \\  | '_ \\
 | |___  | (_) | | (_) | | |_) |
 |_____|  \\___/   \\___/  | .__/
                         |_|
        "
        );
        println!("Welcome to Loop v{}", VERSION);

        if CONFIG.jit_enabled {
            println!(
                "{}You're running Loop in JIT mode. More info: https://looplang.org/docs/internal/jit",
                "WARNING: ".red()
            );
        }

        self.run();

        None
    }

    fn run_code(&mut self, s: String, codegen: &mut CodeGen, line: String) {
        let l = build_lexer(s.as_str());

        let mut p = build_parser(l);

        let program = p.parse();

        if p.errors.is_empty() {
            let mut compiler = build_compiler(self.compiler_state.as_ref());
            let error = compiler.compile(program);

            if error.is_err() {
                let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
                println!("{}", message.red());
                return;
            }

            if !CONFIG.jit_enabled {
                self.compiler_state = Some(compiler.get_state());
            }

            if CONFIG.debug_mode {
                print_instructions(compiler.scope().instructions.clone());
            }

            let mut vm = build_vm(
                compiler.get_bytecode(),
                self.vm_state.as_ref(),
                format!("MAIN{}", line),
            );

            let started = Utc::now();

            let ran = vm.run(codegen);

            let duration = Utc::now().signed_duration_since(started);

            if ran.is_err() {
                println!(
                    "{}",
                    format!("VirtualMachineException: {}", ran.err().unwrap()).red()
                );
            } else {
                if CONFIG.jit_enabled {
                    self.vm_state = Some(vm.get_state());
                }

                if CONFIG.enable_benchmark {
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
        let mut code = "".to_string();

        loop {
            let context = Context::create();
            let module = context.create_module("program");
            let execution_engine = module
                .create_jit_execution_engine(OptimizationLevel::None)
                .ok()
                .ok_or_else(|| "cannot start jit!".to_string())
                .unwrap();

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
                last_popped: None,
            };

            self.line += 1;

            let readline = rl.readline(format!("{} => ", self.line).as_str());

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    if CONFIG.jit_enabled {
                        code.push_str(&*line);
                        code.push('\n');

                        self.run_code(code.clone(), &mut codegen, self.line.to_string());
                    } else {
                        self.run_code(line, &mut codegen, self.line.to_string());
                    }
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
