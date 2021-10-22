use crate::compiler::instructions::Instructions;
use crate::lib::object::function::Function;
use std::borrow::Borrow;

pub struct Frame {
    pub func: Function,
    pub(crate) ip: u32,
    pub base_pointer: i32,
}

pub fn build_frame(func: Function, base: i32) -> Frame {
    Frame {
        func,
        ip: 0,
        base_pointer: base,
    }
}

impl Frame {
    pub(crate) fn instructions(&self) -> &Instructions {
        self.func.func.instructions.borrow()
    }
}
