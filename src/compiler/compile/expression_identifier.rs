use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> CompilerResult {
    let var = compiler
        .variable_scope
        .borrow_mut()
        .resolve(format!("{}{}", compiler.location, identifier.value));

    if let Some(var) = var {
        if var.parameter_id > -1 {
            compiler.add_to_current_function(format!(".LOAD PARAMETER {};", var.parameter_id));
        } else {
            compiler.add_to_current_function(format!(".LOAD VARIABLE {};", var.index));
        }

        return CompilerResult::Success(var._type);
    }

    CompilerResult::Exception(CompilerException::UnknownSymbol(UnknownSymbol {
        name: identifier.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
