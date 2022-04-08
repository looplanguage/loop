use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::expression::identifier::Identifier;
use crate::parser::types::Types;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> CompilerResult {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(identifier.value.as_str());

    if let Some(unwrapped_symbol) = symbol {
        // Only used for compiler defined functions (currently just translated to D std)
        compiler.load_symbol(unwrapped_symbol);
        return CompilerResult::Success(Types::Void);
    } else {
        let var = compiler
            .variable_scope
            .borrow_mut()
            .resolve(format!("{}{}", compiler.location, identifier.value));

        if let Some(var) = var {
            compiler.add_to_current_function(var.transpile());

            return CompilerResult::Success(var._type);
        }
    }

    CompilerResult::Exception(CompilerException::UnknownSymbol(UnknownSymbol {
        name: identifier.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
