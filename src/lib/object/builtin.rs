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

pub const BUILTINS: &[Builtin] = &[builtin!(len)];

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

fn len(arguments: Vec<Rc<Object>>) -> EvalResult {
    match &arguments[0].borrow() {
        Object::String(value) => Ok(Object::Integer(Integer { value: 50 })),
        _ => Ok(Object::Integer(Integer { value: 100 })),
    }
}
