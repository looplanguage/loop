use crate::compiler::CompilerState;
use crate::lib::config::CONFIG;
use crate::lib::util::execute_code;
use crate::vm::VMState;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    line: i32,
    compiler_state: Option<CompilerState>,
    vm_state: Option<VMState>,
}

pub fn build_repl() -> Repl {
    #[cfg(target_os = "windows")]
    control::set_virtual_terminal(true).unwrap();
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
██╗      █████╗  █████╗ ██████╗
██║     ██╔══██╗██╔══██╗██╔══██╗
██║     ██║  ██║██║  ██║██████╔╝
██║     ██║  ██║██║  ██║██╔═══╝
███████╗╚█████╔╝╚█████╔╝██║
╚══════╝ ╚════╝  ╚════╝ ╚═╝
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

        if !CONFIG.jit_enabled {
            self.compiler_state = ran.1;
            self.vm_state = ran.2;
        }

        if let Ok(m) = ran.0 {
            println!("{}", m.borrow().inspect());
        }
    }

    fn run(&mut self) {
        let mut rl = Editor::<()>::new();
        let mut code = "".to_string();

        let mut additive_code = "".to_string();
        let mut level_depth = 0;

        loop {
            self.line += 1;

            let mut adds = String::from("");
            let mut is_line = String::from("=");

            for _ in 0..level_depth {
                adds.push('\x20');
                adds.push('\x20');
                is_line = "#".to_string();
            }

            let readline = rl.readline(format!("{} {}> {}", self.line, is_line, adds).as_str());

            match readline {
                Ok(line) => {
                    if line.as_str() == "exit" {
                        println!("{}", "Exiting the REPL...\n".yellow());
                        break;
                    }
                    rl.add_history_entry(line.as_str());

                    additive_code.push_str(line.as_str());

                    if line.ends_with('{') {
                        level_depth += 1;
                    }

                    if line.contains('}') && level_depth != 0 {
                        level_depth -= 1;
                    }

                    if line.contains('}') {
                        adds = String::from("");
                        for _ in 0..level_depth {
                            adds.push('\x20');
                            adds.push('\x20');
                            is_line = "#".to_string();
                        }

                        let mut spaces = String::from("");
                        for _ in 0..100 {
                            spaces.push(' ');
                        }

                        println!("\u{8}\r{} {}> {}{}{}", self.line, is_line, adds, line, spaces);
                    }

                    if level_depth == 0 {
                        if CONFIG.jit_enabled {
                            code.push_str(&*additive_code);
                            code.push('\n');

                            self.run_code(code.clone());
                        } else {
                            self.run_code(additive_code.clone());
                        }
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
