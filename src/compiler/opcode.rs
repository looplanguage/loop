#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Constant,
    Add,
    Pop,
    Closure,
}
