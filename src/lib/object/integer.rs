use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::object::builtin::EvalResult;
use crate::lib::object::float::Float;
use crate::lib::object::string::LoopString;
use crate::lib::object::{Object, ObjectTrait};

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
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
