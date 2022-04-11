use crate::compiler::{Compiler, CompilerResult};
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_null(_compiler: &mut Compiler) -> CompilerResult {
    _compiler.add_to_current_function("null".to_string());

    CompilerResult::Success(Types::Basic(BaseTypes::Integer))
}
