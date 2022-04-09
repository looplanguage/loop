use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::export::Export;
use crate::parser::types::Types;

pub fn compile_export_statement(_compiler: &mut Compiler, export: Export) -> CompilerResult {
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
        Types::Auto,
        Modifiers::default(),
    );

    _compiler.variable_count += 1;

    _compiler.add_to_current_function(format!("auto {} = ", var.transpile()));

    let variable_borrowed = _compiler
        .variable_scope
        .borrow_mut()
        .get_variable_mutable(var.index, var.name.clone())
        .unwrap()
        .clone();

    let result = _compiler.compile_expression(export.expression, false);

    if let CompilerResult::Success(_type) = result {
        variable_borrowed.as_ref().borrow_mut()._type = _type;
    }

    CompilerResult::Success(Types::Void)
}
