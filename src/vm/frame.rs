use std::borrow::Borrow;
use crate::compiler::instructions::Instructions;
use crate::lib::object::function::Function;

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
    pub(crate) fn instructions(&mut self) -> &Instructions {
        self.func.func.instructions.borrow()
    }
}
