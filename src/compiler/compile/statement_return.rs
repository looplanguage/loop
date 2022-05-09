use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_return_statement(_compiler: &mut Compiler, rt: ReturnStatement) -> CompilerResult {
    if _compiler.scope_index == 0 {
        return CompilerResult::Exception(
            CompilerException::ReturnStatementNotAllowedOutsideFunction,
        );
    }

    _compiler.add_to_current_function(".RETURN {".to_string());

    let result = _compiler.compile_expression(*rt.expression, false);
    _compiler.add_to_current_function("};".to_string());

    #[allow(clippy::single_match)]
    let _type = match &result {
        CompilerResult::Exception(_exception) => return result,
        CompilerResult::Success(_tp) => _tp.clone(),
        _ => return CompilerResult::Exception(CompilerException::Unknown),
    };

    CompilerResult::Success(_type)
}
