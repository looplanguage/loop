use crate::compiler::CompilerState;
use crate::lib::config::CONFIG;
use crate::lib::util::execute_code;
use crate::vm::VMState;
use colored::Colorize;
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
██╗░░░░░░█████╗░░█████╗░██████╗░
██║░░░░░██╔══██╗██╔══██╗██╔══██╗
██║░░░░░██║░░██║██║░░██║██████╔╝
██║░░░░░██║░░██║██║░░██║██╔═══╝░
███████╗╚█████╔╝╚█████╔╝██║░░░░░
╚══════╝░╚════╝░░╚════╝░╚═╝░░░░░
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

    fn run_code(&mut self, s: String) {
        let ran = execute_code(
            s.as_str(),
            self.compiler_state.as_ref(),
            self.vm_state.as_ref(),
        );
        self.compiler_state = ran.1;

        if let Ok(m) = ran.0 {
            println!("{}", m.borrow().inspect());
        }
    }

    fn run(&mut self) {
        let mut rl = Editor::<()>::new();
        let mut code = "".to_string();

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
                    if CONFIG.jit_enabled {
                        code.push_str(&*line);
                        code.push('\n');

                        self.run_code(code.clone());
                    } else {
                        self.run_code(line);
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
