#[derive(Copy, Clone)]
pub enum OpCode {
    Constant,
    Add,
    Pop,
    Closure,
}