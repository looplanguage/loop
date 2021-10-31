use crate::lib::exception::vm::VMException;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
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

pub const BUILTINS: &[Builtin] = &[builtin!(len), builtin!(print), builtin!(println)];

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
}

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
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type for function 'len'. got=\"{:?}\"",
            &arguments[0]
        ))),
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
