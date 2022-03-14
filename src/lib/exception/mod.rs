use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::vm::VMException;
use crate::parser::Parser;

pub mod compiler;
pub mod runtime;
pub mod syntax;
pub mod vm;

#[allow(dead_code)]
pub enum Exception {
    Compiler(CompilerException),
    Parser(String),
    Runtime(RuntimeException),
    VM(VMException),
}
