#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    Constant,
    Add,
    Pop,
    Closure,
    Modulo,
    Multiply,
    Divide,
    Minus,
    SetVar,
    GetVar,
    Equals,
    NotEquals,
    GreaterThan,
    Jump,
    JumpIfFalse,
    Return,
}
