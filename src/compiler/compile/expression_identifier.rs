use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> Option<CompilerException> {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(identifier.value.as_str());

    if let Some(unwrapped_symbol) = symbol {
        compiler.load_symbol(unwrapped_symbol);
        return None;
    } else {
        let var = compiler
            .variable_scope
            .borrow_mut()
            .resolve(format!("{}{}", compiler.location, identifier.value));

        if var.is_some() {
            compiler.emit(OpCode::GetVar, vec![var.unwrap().index]);
            return None;
        }
    }

    Some(CompilerException::UnknownSymbol(UnknownSymbol {
        name: identifier.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
