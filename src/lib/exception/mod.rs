use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::parser::ParserException;
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::vm::VMException;

pub mod compiler;
pub mod parser;
pub mod runtime;
pub mod vm;

#[allow(dead_code)]
pub enum Exception {
    Compiler(CompilerException),
    Parser(ParserException),
    Runtime(RuntimeException),
    VM(VMException),
}
