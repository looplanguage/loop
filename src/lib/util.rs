use crate::lib::config::CONFIG;
use crate::lib::flags;
use sanzio::parse_multivalue;
use std::env;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use vinci::types::ValueType;

pub fn print_valuetype(value_type: ValueType) {
    match value_type {
        ValueType::Integer(c) => print!("{}", c),
        ValueType::Boolean(c) => print!("{}", c),
        ValueType::Character(c) => {
            print!("{}", c)
        }
        ValueType::Array(arr) => {
            if let Some(first) = arr.first() {
                if let ValueType::Character(_) = first.clone() {}
            }
        }
        ValueType::Void => print!("null"),
        ValueType::Float(f) => print!("{}", f),
        ValueType::Compound(_, _) => print!("awh")
    }
}

pub fn get_flags() -> flags::Flags {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let mut flags = flags::build_flags();
    flags.parse_flags(args);
    flags
}

pub fn run_file(path: String) {
    let file = std::fs::File::open(Path::new(path.as_str()));

    if let Err(err) = file {
        println!("{}", err);
        exit(1);
    }

    let mut file = file.unwrap();
    let mut content = String::new();
    let result = file.read_to_string(&mut content);

    if let Err(err) = result {
        println!("{}", err);
        exit(1);
    }

    let arc = picasso::compile(content.as_str());
    if CONFIG.debug_mode {
        println!("Arc\n#---------\n{}\n---------#", arc.0);
    }

    let ast = vinci::parse(&*arc.0);

    if CONFIG.debug_mode {
        println!("AST\n#---------\n{}\n---------#", ast);
    }

    let mut backend = unsafe { sanzio::Sanzio::new() };

    if CONFIG.debug_mode {
        println!(
            "Lua\n#---------\n{}\n---------#",
            sanzio::Sanzio::compile_to_lua(&ast)
        );
    }

    let result = backend.run(ast);
    let multivalue = parse_multivalue(result.clone());

    if multivalue != ValueType::Void {
        print_valuetype(parse_multivalue(result.clone()));
        println!();
    }
}
