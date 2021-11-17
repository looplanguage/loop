use crate::lib::object::{Object, ObjectTrait};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Vec<u8>,
    pub num_locals: u8,
    pub num_parameters: u8,
    pub parsed_function: Option<crate::parser::expression::function::Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub func: CompiledFunction,
    pub free: Vec<Rc<RefCell<Object>>>,
}

impl ObjectTrait for CompiledFunction {
    fn inspect(&self) -> String {
        format!("CompiledFunction[{:p}]", self)
    }
}

impl ObjectTrait for Function {
    fn inspect(&self) -> String {
        format!("Function[{}]", self.func.num_parameters)
    }
}
