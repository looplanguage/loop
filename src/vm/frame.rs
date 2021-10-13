use crate::compiler::instructions::Instructions;
use crate::object::function::Function;

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
    pub(crate) fn instructions(&mut self) -> &mut Instructions {
        self.func.func.instructions.as_mut()
    }
}
