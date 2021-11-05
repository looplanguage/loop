use std::borrow::BorrowMut;
use crate::lib::object::{boolean, integer, Object, ObjectTrait};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::lib::exception::vm::VMException;
use crate::lib::object::builtin::EvalResult;

#[derive(Clone, Debug, PartialEq)]
pub struct Array{
    pub(crate) values: Vec<Rc<RefCell<Object>>>,
}

impl ObjectTrait for Array {
    fn inspect(&self) -> String {
        let mut items: Vec<String> = Vec::new();

        for value in &self.values {
            items.push(value.borrow().inspect());
        }

        format!("[{}]", items.join(", "))
    }
}
