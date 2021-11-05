use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::exception::vm::VMException;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use crate::lib::object::string::LoopString;

pub type EvalResult = Result<Object, VMException>;
pub type BuiltinFunction = fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult;

pub struct Builtin {
    pub name: &'static str,
    pub builtin: BuiltinFunction,
}

macro_rules! extension {
    ($name:ident) => {
        Builtin {
            name: stringify!($name),
            builtin: ($name),
        }
    };
}

pub const EXTENSION_METHODS: &[Builtin] = &[extension!(to_string), extension!(to_int)];

pub fn lookup(name: &str) -> Option<u32> {
    if name == "null" {
        return None;
    }

    let mut i = 0;
    for b in EXTENSION_METHODS {
        if b.name == name {
            return Some(i);
        }

        i += 1;
    }
    None
}

fn to_string(extending: Rc<RefCell<Object>>, arguments: Vec<Object>) -> EvalResult {
    return match &*extending.as_ref().borrow() {
        Object::Integer(integer) => {
            Ok(Object::String(LoopString {
                value: integer.value.to_string()
            }))
        }
        _ => {
            Ok(Object::Null(Null {}))
        }
    }
}


fn to_int(extending: Rc<RefCell<Object>>, arguments: Vec<Object>) -> EvalResult {
    return match &*extending.as_ref().borrow() {
        Object::String(string) => {
            let parsed = string.value.parse::<i64>();

            if parsed.is_err() {
                return Err(VMException::CannotParseInt(format!("unable to parse. got=\"{:?}\"", extending)))
            }

            Ok(Object::Integer(Integer {
                value: parsed.unwrap()
            }))
        }
        _ => {
            Ok(Object::Null(Null {}))
        }
    }
}