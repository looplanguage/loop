use crate::lib::config::CONFIG;
use crate::lib::exception::Exception;
use crate::{compiler, lexer, parser};
use chrono::{Local, Utc};
use colored::Colorize;
use dirs::home_dir;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, Output};

type ExecuteCodeReturn = Result<String, String>;

/// Executes Loop code, the process is as follows:
/// - Lex input string
/// - Parse lexed tokens
/// - Transpile to D
/// - Call D compiler or if not found copy it to the Loop tmp directory
/// - Execute D compiler
/// - Run executable
/// - Return stdout (or stderr) to the user
///
/// Usage:
/// ```
/// let executed = execute_code("println(\"Hello World!\"");
///
/// if executed.is_err() {
///     // Handle error
/// }
/// ```
pub fn execute_code(code: &str) -> ExecuteCodeReturn {
    let l = lexer::build_lexer(code);
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            if let Exception::Syntax(msg) = err {
                println!("ParserException: {}", msg);
            }
        }

        panic!("Parser exceptions occurred!")
    }

    let mut comp = compiler::Compiler::default();
    let error = comp.compile(program);

    let mut imports = String::new();

    for import in comp.imports.clone() {
        imports.push_str(&*format!("import {};", import));
    }

    let mut code = String::new();

    code.push_str(imports.as_str());

    for function in comp.functions {
        code.push_str(function.1.code.as_str());
    }

    // Write output to temp directory in Loop home directory
    let home_dir = home_dir().unwrap();
    let mut dir = home_dir.to_str().unwrap().to_string();
    dir.push_str("/.loop/tmp/");

    if !Path::new(&*dir.clone()).exists() {
        let result = create_dir(dir.clone());
        if let Err(result) = result {
            return Err(result.to_string());
        }
    }

    let loop_path = std::env::current_exe().unwrap();
    let loop_path = loop_path.parent().unwrap().to_str().unwrap();

    let filename = format!("{}", Local::now().format("loop_%Y%m%d%H%M%S%f"));

    if !CONFIG.debug_mode {
        let file = File::create(format!("{}{}.d", dir, filename));
        let result = file.unwrap().write_all(code.as_bytes());

        if let Err(result) = result {
            return Err(result.to_string());
        }
    } else {
        println!("{}", code);
    }

    if error.is_err() {
        let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
        println!("{}", message.as_str().red());
        return Err(message);
    }

    let started = Utc::now();

    let mut output: Option<Output> = None;

    if let Some(dpath) = CONFIG.dcompiler.clone() {
        let mut path = dpath.clone();

        if dpath.is_empty() {
            path = "dmd".to_string();
        }

        #[cfg(unix)]
        Command::new(path.as_str())
            .args([
                format!("{}{}.d", dir, filename),
                format!("-of={}{}", dir, filename),
            ])
            .output()
            .expect(&*format!("failed to run D compiler! ({})", path.as_str()));

        #[cfg(windows)]
        Command::new(path.as_str())
            .args([
                format!("{}{}.d", dir, filename),
                format!("-of={}{}.exe", dir, filename),
            ])
            .output()
            .expect(&*format!("failed to run D compiler! ({})", path.as_str()));

        #[cfg(windows)]
        let name = format!("{}{}.exe", dir, filename);

        #[cfg(unix)]
        let name = format!("{}{}.exe", dir, filename);

        output = Some(Command::new(name.as_str()).output().expect(&*format!(
            "Unable to run Loop program at: {}{}",
            dir, filename
        )));
    };

    // Compile it & execute (only on macos and arm)
    if !CONFIG.debug_mode && CONFIG.dcompiler.is_none() {
        output = Option::from(if cfg!(all(target_os = "macos")) {
            let result =
                Command::new(format!("{}dmd-latest-mac/dmd2/osx/bin/dmd", loop_path).as_str())
                    .args([
                        format!("{}{}.d", dir, filename),
                        format!("-of={}{}", dir, filename),
                    ])
                    .output()
                    .expect(&*format!(
                        "failed to run D compiler! ({}dmd-latest-mac/dmd2/osx/bin/dmd)",
                        loop_path
                    ));

            if !result.status.success() {
                result
            } else {
                Command::new(format!("{}{}", dir, filename))
                    .output()
                    .expect(&*format!(
                        "Unable to run Loop program at: {}{}",
                        dir, filename
                    ))
            }
        } else if cfg!(all(target_os = "windows")) {
            let result = Command::new(
                format!("{}dmd-latest-win64/dmd2/windows/bin64/dmd.exe", loop_path).as_str(),
            )
            .args([
                format!("{}{}.d", dir, filename),
                format!("-of={}{}.exe", dir, filename),
            ])
            .output()
            .expect("failed to run D compiler! (dmd)");

            if !result.status.success() {
                result
            } else {
                Command::new(format!("{}{}.exe", dir, filename))
                    .output()
                    .expect(&*format!(
                        "Unable to run Loop program at: {}{}",
                        dir, filename
                    ))
            }
        } else if cfg!(unix) {
            let result = Command::new(
                format!("{}dmd-latest-linux/dmd2/linux/bin64/dmd", loop_path).as_str(),
            )
            .args([
                format!("{}{}.d", dir, filename),
                format!("-of={}{}.exe", dir, filename),
            ])
            .output()
            .expect("failed to run D compiler! (dmd)");

            if !result.status.success() {
                result
            } else {
                Command::new(format!("{}{}.exe", dir, filename))
                    .output()
                    .expect(&*format!(
                        "Unable to run Loop program at: {}{}",
                        dir, filename
                    ))
            }
        } else {
            panic!("Unsupported platform!");
        });
    }

    if let Some(output) = output {
        if !output.status.success() {
            println!("{}", String::from_utf8_lossy(&*output.stderr));
            exit(output.status.code().unwrap());
        } else {
            print!("{}", String::from_utf8_lossy(&*output.stdout));
        }
    }

    let duration = Utc::now().signed_duration_since(started);

    if CONFIG.enable_benchmark {
        let formatted = duration.to_string().replace("PT", "");
        println!("Execution Took: {}", formatted);
    }

    Ok(String::from(""))
}
