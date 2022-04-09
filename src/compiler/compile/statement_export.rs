use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::export::Export;

pub fn compile_export_statement(_compiler: &mut Compiler, _export: Export) -> CompilerResult {
    /*if _compiler.export_name.is_empty() {
        // TODO: Return error
        return CompilerResult::Exception(CompilerException::Unknown);
    }

    let var = _compiler.define_variable(
        format!(
            "{}{}",
            _compiler.prev_location,
            _compiler.export_name.clone()
        ),
        Types::Auto,
    );

    println!("Setting variable in: \"{}\"", _compiler.prev_location);

    _compiler.add_to_current_function(format!("auto {} = ", var.transpile()));

    let variable_borrowed = _compiler
        .variable_scope
        .borrow_mut()
        .get_variable_mutable(var.index, var.name)
        .unwrap();

    let result = _compiler.compile_expression(export.expression, false);

    if let CompilerResult::Success(_type) = result {
        variable_borrowed.as_ref().borrow_mut()._type = _type;
    }*/

    CompilerResult::Exception(CompilerException::Unknown)
}
