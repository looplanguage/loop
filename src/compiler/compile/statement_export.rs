use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::export::Export;

pub fn compile_export_statement(
    _compiler: &mut Compiler,
    export: Export,
) -> CompilerResult {
    if _compiler.export_name.is_empty() {
        // TODO: Return error
        return CompilerResult::Exception(CompilerException::Unknown);
    }

    let var = _compiler.variable_scope.borrow_mut().define(
        _compiler.variable_count,
        format!(
            "{}{}",
            _compiler.prev_location,
            _compiler.export_name.clone()
        ),
        export.expression.clone(),
    );

    _compiler.variable_count += 1;

    _compiler.compile_expression(export.expression);

    _compiler.emit(OpCode::SetVar, vec![var.index]);

    CompilerResult::Success
}
