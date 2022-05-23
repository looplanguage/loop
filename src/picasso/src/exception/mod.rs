//! Exceptions that can be thrown by Loop

pub mod compiler;
pub mod compiler_new;
pub mod flag;
pub mod runtime;
pub mod syntax;
pub mod vm;

#[allow(dead_code)]
pub enum Exception {
    Compiler(compiler::CompilerException),
    Syntax(String),
    Runtime(runtime::RuntimeException),
    VM(vm::VMException),
}
