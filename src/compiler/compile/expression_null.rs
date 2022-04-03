use crate::compiler::{Compiler, CompilerResult};

pub fn compile_expression_null(_compiler: &mut Compiler) -> CompilerResult {
    CompilerResult::Success
}
