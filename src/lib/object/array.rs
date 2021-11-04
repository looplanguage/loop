use std::borrow::BorrowMut;
use crate::lib::object::{boolean, Object, ObjectTrait};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::lib::exception::vm::VMException;
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
            // 1: remove
            1 => Some(Box::new(remove(self))),
            // 2: slice
            2 => Some(Box::new(add(self))),
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
        if let Object::Array(ref mut arr) = &mut *_mut_arr.as_ref().borrow_mut() {
            for _arg in _args {
                arr.values.push(Rc::from(RefCell::from(_arg)));
            }
        }

        Ok(Object::Boolean(boolean::Boolean { value: true }))
    }
}

// 1: remove(index: Integer)
pub fn remove(arr: &Array) -> impl Fn(Rc<RefCell<Object>>, Vec<Object>) -> EvalResult {
    move |_mut_arr, _args| -> EvalResult {
        if _args.len() != 1 {
            return Err(VMException::IncorrectArgumentCount(1, _args.len() as i32));
        }

        let mut removed: Object = Object::Boolean(boolean::Boolean { value: false });
        if let Object::Array(ref mut arr) = &mut *_mut_arr.as_ref().borrow_mut() {
            if let Object::Integer(integer) = _args[0] {
                if arr.values.len() - 1 > integer.value as usize {
                    removed = arr.values.remove(integer.value as usize).as_ref().borrow().clone();
                } else {
                    return Err(VMException::EmptyArray);
                }
            } else {
                return Err(VMException::IncorrectType(format!("wrong type. expected=\"Integer\". got=\"{:?}\"", _args[0])));
            }
        }

        return Ok(removed);
    }
}
