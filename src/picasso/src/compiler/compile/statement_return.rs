use crate::compiler::Compiler;
use crate::exception::compiler::{CompilerException, CompilerExceptionCode};
use crate::parser::statement::return_statement::ReturnStatement;
use crate::parser::types::Types;

pub fn compile_return_statement(
    _compiler: &mut Compiler,
    rt: ReturnStatement,
) -> Result<Types, CompilerException> {
    if _compiler.scope_index == 0 {
        return Err(CompilerException::new(
            0,
            0,
            CompilerExceptionCode::ReturnStatementNotAllowedOutsideFunction,
        ));
    }

    _compiler.add_to_current_function(".RETURN {".to_string());

    let result = _compiler.compile_expression(*rt.expression);
    _compiler.add_to_current_function("};".to_string());

    #[allow(clippy::single_match)]
    let _type = match &result {
        Err(_exception) => return result,
        Ok(_tp) => _tp.clone(),
    };

    Ok(_type)
}
