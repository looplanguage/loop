use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::exception::vm::VMException;
use crate::lib::object::builtin::EvalResult;
use crate::lib::object::integer::Integer;
use crate::lib::object::{Object, ObjectTrait};

#[derive(Clone, Debug, PartialEq)]
pub struct LoopString {
    pub(crate) value: String,
}

impl ObjectTrait for LoopString {
    fn inspect(&self) -> String {
        format!("\"{}\"", self.value.clone())
    }
}

impl LoopString {
    pub fn get_extension(&self, extension: i32) -> Option<Box<dyn Fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult>> {
        match extension {
            // to_int
            0 => Some(Box::from(to_int(self.value.clone()))),
            _ => None,
        }
    }
}

// Extension methods

// 0: to_int()
pub fn to_int(value: String) -> impl Fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult {
    move |_mut_str, _args| -> EvalResult {
        let new_value = value.parse::<i64>();

        if new_value.is_err() {
            return Err(VMException::CannotParseInt(value.clone()));
        }

        Ok(Object::Integer(Integer {
            value: new_value.unwrap(),
        }))
    }
}
