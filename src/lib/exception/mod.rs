use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::vm::VMException;

pub mod compiler;
mod parser;
pub mod runtime;
pub mod vm;

#[allow(dead_code)]
pub enum Exception {
    Compiler(CompilerException),
    Parser(String),
    Runtime(RuntimeException),
    VM(VMException),
}
