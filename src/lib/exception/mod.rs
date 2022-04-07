//! Exceptions that can be thrown by Loop
use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::vm::VMException;

pub mod compiler;
pub mod flag;
pub mod runtime;
pub mod syntax;
pub mod vm;
pub mod compiler_new;

#[allow(dead_code)]
pub enum Exception {
    Compiler(CompilerException),
    Syntax(String),
    Runtime(RuntimeException),
    VM(VMException),
}
