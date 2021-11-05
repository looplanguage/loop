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
