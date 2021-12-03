use crate::lib::exception::vm::VMException;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::string::LoopString;
use crate::lib::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub type EvalResult = Result<Object, VMException>;
pub type BuiltinFunction = fn(Vec<Rc<RefCell<Object>>>) -> EvalResult;

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

pub const BUILTINS: &[Builtin] = &[
    builtin!(len),
    builtin!(print),
    builtin!(println),
    builtin!(format),
];

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

fn print(arguments: Vec<Rc<RefCell<Object>>>) -> EvalResult {
    for argument in arguments {
        match &*argument.as_ref().borrow() {
            Object::String(str) => {
                print!("{}", str.value);
            }
            _ => {
                print!("{}", argument.as_ref().borrow().inspect());
            }
        }
    }

    Ok(Object::Null(Null {}))
}

fn println(arguments: Vec<Rc<RefCell<Object>>>) -> EvalResult {
    for argument in arguments {
        match &*argument.as_ref().borrow() {
            Object::String(str) => {
                println!("{}", str.value);
            }
            _ => {
                println!("{}", argument.as_ref().borrow().inspect());
            }
        }
    }

    Ok(Object::Null(Null {}))
}

fn len(arguments: Vec<Rc<RefCell<Object>>>) -> EvalResult {
    check_length(arguments.clone(), 1)?;

    match &*arguments[0].as_ref().borrow() {
        Object::String(string) => Ok(Object::Integer(Integer {
            value: string.value.len() as i64,
        })),
        Object::Array(array) => Ok(Object::Integer(Integer {
            value: array.values.len() as i64,
        })),
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type for function 'len'. got=\"{:?}\"",
            &arguments[0]
        ))),
    }
}

fn format(arguments: Vec<Rc<RefCell<Object>>>) -> EvalResult {
    if arguments.len() < 2 {
        return Err(VMException::IncorrectArgumentCount(
            2 as i32,
            arguments.len() as i32,
        ));
    } else {
        return if let Object::String(string) = &*arguments[0].as_ref().borrow() {
            let mut copy = string.value.clone();

            for i in 1..arguments.len() {
                match &*arguments[i].as_ref().borrow() {
                    Object::Integer(int) => {
                        copy = copy.replacen("%a", int.value.to_string().as_str(), 1)
                    }
                    Object::Boolean(boolean) => {
                        copy = copy.replacen("%a", boolean.value.to_string().as_str(), 1)
                    }
                    Object::Null(_) => copy = copy.replacen("%a", "null", 1),
                    Object::Float(float) => {
                        copy = copy.replacen("%a", float.value.to_string().as_str(), 1)
                    }
                    Object::String(str) => {
                        copy = copy.replacen("%a", str.value.to_string().as_str(), 1)
                    }
                    _ => {
                        return Err(VMException::IncorrectType(format!(
                            "incorrect type for function 'format'. got=\"{:?}\"",
                            &arguments[i]
                        )));
                    }
                }
            }

            Ok(Object::String(LoopString { value: copy }))
        } else {
            Err(VMException::IncorrectType(format!(
                "incorrect type for function 'format'. got=\"{:?}\"",
                &arguments[0]
            )))
        };
    }
}

fn check_length(args: Vec<Rc<RefCell<Object>>>, required_length: usize) -> EvalResult {
    if args.len() != required_length {
        return Err(VMException::IncorrectArgumentCount(
            required_length as i32,
            args.len() as i32,
        ));
    }

    Ok(Object::Null(Null {}))
}
