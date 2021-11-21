#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    Constant,      // 0
    Add,           // 1
    Pop,           // 2
    Closure,       // 3
    Modulo,        // 4
    Multiply,      // 5
    Divide,        // 6
    Minus,         // 7
    SetVar,        // 8
    GetVar,        // 9
    Equals,        // 10
    NotEquals,     // 11
    GreaterThan,   // 12
    Jump,          // 13
    JumpIfFalse,   // 14
    Return,        // 15
    Function,      // 16
    Call,          // 17
    GetLocal,      // 18
    GetFree,       // 19
    GetBuiltin,    // 20
    CallExtension, // 21
    Array,         // 22
    Index,         // 23
    AssignIndex,   // 24
    Hashmap,       // 25
    Pow,           // 26
    Enum,          // 27
}
