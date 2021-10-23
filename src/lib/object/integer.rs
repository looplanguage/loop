use crate::lib::object::builtin::{Builtin, BuiltinFunction, EvalResult};
use crate::lib::object::float::Float;
use crate::lib::object::null::Null;
use crate::lib::object::string::LoopString;
use crate::lib::object::{Object, ObjectTrait};
use std::env::args;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Integer {
    pub(crate) value: i64,
}

impl Integer {
    pub fn to_float(self) -> Float {
        Float {
            value: self.value as f64,
        }
    }

    pub fn get_extension(&self, extension: i32) -> Option<Box<dyn Fn(Vec<Object>) -> EvalResult>> {
        match extension {
            // to_string
            0 => Some(Box::from(to_string(self.value))),
            _ => None,
        }
    }

    pub fn to_string(&self, args: Vec<Rc<Object>>) -> EvalResult {
        Ok(Object::String(LoopString {
            value: self.value.to_string(),
        }))
    }
}

pub fn to_string(value: i64) -> impl Fn(Vec<Object>) -> EvalResult {
    move |args| -> EvalResult {
        Ok(Object::String(LoopString {
            value: value.to_string(),
        }))
    }
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
