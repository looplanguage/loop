use crate::compiler::CompilerState;
use crate::lib::config::CONFIG;
use crate::lib::exception::Exception;
use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use crate::{compiler, lexer, parser};
use chrono::{Local, Utc};
use colored::Colorize;
use dirs::home_dir;
use std::cell::RefCell;
use std::fs::{create_dir, create_dir_all, File, remove_dir_all};
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use std::rc::Rc;

type ExecuteCodeReturn = (Result<Rc<RefCell<Object>>, String>, Option<CompilerState>);

pub fn execute_code(code: &str, compiler_state: Option<&CompilerState>) -> ExecuteCodeReturn {
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

    let mut comp = compiler::build_compiler(compiler_state);
    let error = comp.compile(program);

    let mut imports = String::new();

    for import in comp.imports.clone() {
        imports.push_str(&*format!("import {};", import));
    }

    let mut code = String::new();

    code.push_str(imports.as_str());

    for function in comp.functions.clone() {
        code.push_str(function.1.as_str());
    }

    // Write output to temp directory in Loop home directory
    let home_dir = home_dir().unwrap();
    let mut dir = home_dir.to_str().unwrap().to_string();
    dir.push_str("/.loop/tmp/");

    if !Path::new(&*dir.clone()).exists() {
        let result = create_dir(dir.clone());
        if let Err(result) = result {
            return (Err(result.to_string()), None);
        }
    }

    let filename = format!("{}", Local::now().format("loop_%Y%m%d%H%M%S%f"));
    let file = File::create(format!("{}{}.d", dir, filename));
    let result = file.unwrap().write_all(code.as_bytes());

    if let Err(result) = result {
        return (Err(result.to_string()), None);
    }

    if error.is_err() {
        let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
        println!("{}", message.as_str().red());
        return (Err(message), None);
    }

    let started = Utc::now();

    // Compile it & execute (only on macos and arm)
    let output = if cfg!(all(target_os = "macos")) {
        let result = Command::new("ldc2")
            .args([format!("{}{}.d", dir, filename), format!("--of={}{}", dir, filename)])
            .output()
            .expect("failed to run D compiler! (ldc2)");

        if !result.status.success() {
            result
        } else {
            Command::new(format!("{}{}", dir, filename))
                .output()
                .expect(&*format!("Unable to run Loop program at: {}{}", dir, filename))
        }
    } else {
        let result = Command::new("dmd")
            .args([format!("{}main.d", dir), format!("-of={}main", dir)])
            .output()
            .expect("failed to run D compiler! (dmd)");

        if !result.status.success() {
            result
        } else {
            Command::new(format!("{}{}", dir, filename))
                .output()
                .expect(&*format!("Unable to run Loop program at: {}{}", dir, filename))
        }
    };

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&*output.stderr));
        exit(output.status.code().unwrap());
    } else {
        print!("{}", String::from_utf8_lossy(&*output.stdout));
    }

    let duration = Utc::now().signed_duration_since(started);

    if CONFIG.enable_benchmark {
        let formatted = duration.to_string().replace("PT", "");
        println!("Execution Took: {}", formatted);
    }

    (
        Ok(Rc::from(RefCell::from(Object::Integer(Integer {
            value: 0,
        })))),
        Some(comp.get_state()),
    )
}
