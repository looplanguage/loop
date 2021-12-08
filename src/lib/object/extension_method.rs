use crate::lib::exception::vm::VMException;
use crate::lib::object::integer::Integer;
use crate::lib::object::string::LoopString;
use crate::lib::object::{boolean, Object};
use std::cell::RefCell;
use std::rc::Rc;

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

pub const EXTENSION_METHODS: &[Builtin] = &[
    extension!(to_string),
    extension!(to_int),
    extension!(add),
    extension!(remove),
    extension!(slice),
    extension!(length),
];

pub fn lookup(name: &str) -> Option<u32> {
    if name == "null" {
        return None;
    }

    for (i, b) in EXTENSION_METHODS.iter().enumerate() {
        if b.name == name {
            return Some(i as u32);
        }
    }
    None
}

fn to_string(extending: Rc<RefCell<Object>>, _arguments: Vec<Object>) -> EvalResult {
    return match &*extending.as_ref().borrow() {
        Object::Integer(integer) => Ok(Object::String(LoopString {
            value: integer.value.to_string(),
        })),
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    };
}

fn to_int(extending: Rc<RefCell<Object>>, _arguments: Vec<Object>) -> EvalResult {
    return match &*extending.as_ref().borrow() {
        Object::String(string) => {
            let parsed = string.value.parse::<i64>();

            if parsed.is_err() {
                return Err(VMException::CannotParseInt(format!(
                    "unable to parse. got=\"{:?}\"",
                    extending
                )));
            }

            Ok(Object::Integer(Integer {
                value: parsed.unwrap(),
            }))
        }
        Object::Boolean(boolean) => {
            if boolean.value == true {
                Ok(Object::Integer(Integer { value: 1 }))
            } else {
                Ok(Object::Integer(Integer { value: 0 }))
            }
        }
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    };
}

pub fn add(extending: Rc<RefCell<Object>>, arguments: Vec<Object>) -> EvalResult {
    match &mut *extending.as_ref().borrow_mut() {
        Object::Array(ref mut arr) => {
            for arg in arguments {
                arr.values.push(Rc::from(RefCell::from(arg)));
            }

            Ok(Object::Boolean(boolean::Boolean { value: true }))
        }
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    }
}

pub fn remove(extending: Rc<RefCell<Object>>, arguments: Vec<Object>) -> EvalResult {
    match &mut *extending.as_ref().borrow_mut() {
        Object::Array(ref mut arr) => {
            if arguments.len() != 1 {
                return Err(VMException::IncorrectArgumentCount(
                    1,
                    arguments.len() as i32,
                ));
            }

            return if let Object::Integer(integer) = arguments[0] {
                if arr.values.len() > integer.value as usize {
                    Ok(arr
                        .values
                        .remove(integer.value as usize)
                        .as_ref()
                        .borrow()
                        .clone())
                } else {
                    Err(VMException::EmptyArray)
                }
            } else {
                Err(VMException::IncorrectType(format!(
                    "wrong type. expected=\"Integer\". got=\"{:?}\"",
                    arguments[0]
                )))
            };
        }
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    }
}

pub fn slice(extending: Rc<RefCell<Object>>, arguments: Vec<Object>) -> EvalResult {
    match &mut *extending.as_ref().borrow_mut() {
        Object::Array(ref mut arr) => {
            if arguments.len() != 2 {
                return Err(VMException::IncorrectArgumentCount(
                    2,
                    arguments.len() as i32,
                ));
            }

            if let Object::Integer(start) = arguments[0] {
                if let Object::Integer(end) = arguments[1] {
                    // 3, 0, 1
                    if start.value <= end.value
                        && arr.values.len() > start.value as usize
                        && arr.values.len() > end.value as usize
                    {
                        arr.values =
                            arr.values[start.value as usize..(end.value + 1) as usize].to_owned()
                    } else {
                        return Err(VMException::EmptyArray);
                    }
                } else {
                    return Err(VMException::IncorrectType(format!(
                        "wrong type. expected=\"Integer\". got=\"{:?}\"",
                        arguments[0]
                    )));
                }
            } else {
                return Err(VMException::IncorrectType(format!(
                    "wrong type. expected=\"Integer\". got=\"{:?}\"",
                    arguments[0]
                )));
            }

            Ok(Object::Boolean(boolean::Boolean { value: true }))
        }
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    }
}

fn length(extending: Rc<RefCell<Object>>, _arguments: Vec<Object>) -> EvalResult {
    return match &*extending.as_ref().borrow() {
        Object::Array(arr) => Ok(Object::Integer(Integer {
            value: arr.values.len() as i64,
        })),
        Object::String(str) => Ok(Object::Integer(Integer {
            value: str.value.len() as i64,
        })),
        _ => Err(VMException::IncorrectType(format!(
            "incorrect type. got=\"{:?}\"",
            extending
        ))),
    };
}
