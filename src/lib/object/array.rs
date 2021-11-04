use std::borrow::BorrowMut;
use crate::lib::object::{boolean, Object, ObjectTrait};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::lib::object::builtin::EvalResult;

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub(crate) values: Vec<Rc<RefCell<Object>>>,
}

impl Array {
    pub fn get_extension(&self, extension: i32) -> Option<Box<dyn Fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult>> {
        match extension {
            // 0: add
            0 => Some(Box::new(add(self))),
            _ => None,
        }
    }
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

// Extension methods

// 0: add(obj: Object)
pub fn add(arr: &Array) -> impl Fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult {
    move |_mut_arr, _args| -> EvalResult {
        println!("CALLED!");
        if let Object::Array(ref mut arr) = &mut *_mut_arr.as_ref().borrow_mut() {
            println!("{:?}", arr.values);

            for _arg in _args {
                arr.values.push(Rc::from(RefCell::from(_arg)));
            }
        }

        Ok(Object::Boolean(boolean::Boolean { value: true }))
    }
}
