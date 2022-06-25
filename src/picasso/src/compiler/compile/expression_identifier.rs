use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> CompilerResult {
    let var = compiler.resolve_variable(&identifier.value);

    if let Some(var) = var {
        if var.parameter_id > -1 {
            compiler.add_to_current_function(format!(
                ".LOAD PARAMETER {} {};",
                var.function_identifier, var.parameter_id
            ));
        } else {
            // Two colons means that the identifier is pointing towards an imported module, here we
            // check if that identifier is public in that module and if it is not we return an error
            if identifier.value.contains("::")
                && compiler.location != var.modifiers.module
                && !var.modifiers.public
            {
                return CompilerResult::Exception(CompilerException::NotPublic(
                    var.modifiers.module,
                    var.name,
                ));
            }

            compiler.add_to_current_function(format!(".LOAD VARIABLE {};", var.index));
        }

        return CompilerResult::Success(var._type);
    }

    CompilerResult::Exception(CompilerException::UnknownSymbol(UnknownSymbol {
        name: identifier.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
