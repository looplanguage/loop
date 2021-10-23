use crate::lib::exception::vm::VMException;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use std::borrow::Borrow;
use std::rc::Rc;

pub type EvalResult = Result<Object, VMException>;
pub type BuiltinFunction = fn(Vec<Rc<Object>>) -> EvalResult;

pub struct Builtin {
    pub name: &'static str,
    pub builtin: Object,
}

macro_rules! builtin {
    ($name:ident) => {
        Builtin {
            name: stringify!($name),
            builtin: Object::Builtin($name),
        }
    };
}

pub const BUILTINS: &[Builtin] = &[builtin!(len), builtin!(print), builtin!(println)];

/*
pub fn lookup(name: &str) -> Option<Object> {
    if name == "null" {
        return Some(Object::Null(Null {}));
    }

    for b in BUILTINS {
        if b.name == name {
            return Some(b.builtin.clone());
        }
    }
    None
}*/

fn print(arguments: Vec<Rc<Object>>) -> EvalResult {
    for argument in arguments {
        match &*argument {
            Object::String(str) => {
                print!("{}", str.value);
            }
            _ => {
                print!("{}", argument.inspect());
            }
        }
    }

    Ok(Object::Null(Null {}))
}

fn println(arguments: Vec<Rc<Object>>) -> EvalResult {
    for argument in arguments {
        match &*argument {
            Object::String(str) => {
                println!("{}", str.value);
            }
            _ => {
                println!("{}", argument.inspect());
            }
        }
    }

    Ok(Object::Null(Null {}))
}

fn len(arguments: Vec<Rc<Object>>) -> EvalResult {
    check_length(arguments.clone(), 1)?;

    match &arguments[0].borrow() {
        Object::String(string) => Ok(Object::Integer(Integer {
            value: string.value.len() as i64,
        })),
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type for function 'len'. got=\"{:?}\"",
            &arguments[0]
        ))),
    }
}

fn check_length(args: Vec<Rc<Object>>, required_length: usize) -> EvalResult {
    if args.len() != required_length {
        return Err(VMException::IncorrectArgumentCount(
            required_length as i32,
            args.len() as i32,
        ));
    }

    Ok(Object::Null(Null {}))
}
