use crate::lib::config::CONFIG;
use crate::lib::util::print_valuetype;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use sanzio::parse_multivalue;
use std::process::ExitCode;
use vinci::types::ValueType;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn start() -> Result<(), ExitCode> {
    let mut backend = unsafe { sanzio::Sanzio::new() };
    let mut compiler_state: Option<picasso::compiler::CompilerState> = None;
    let mut rl = Editor::<()>::new();

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
    println!(
        "Welcome to Loop v{}, more info: https://looplang.org/docs/intro",
        VERSION
    );

    if CONFIG.debug_mode {
        println!("Debug mode enabled!");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let result = if let Some(compiler_state) = compiler_state.clone() {
                    picasso::compile_with_state(line.as_str(), compiler_state)
                } else {
                    picasso::compile(line.as_str(), None)?
                };

                if CONFIG.debug_mode {
                    println!("Arc\n{}", result.0.as_str());
                }

                let ast = vinci::parse(result.0.as_str());

                let evaluated = backend.run(ast);
                let parsed = parse_multivalue(evaluated);

                compiler_state = Some(result.1);

                print_valuetype(parsed.clone());

                if parsed != ValueType::Void {
                    println!();
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
