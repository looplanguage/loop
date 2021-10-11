use crate::compiler::Compiler;
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> Option<String> {
    let symbol = {
        match compiler
            .symbol_table
            .borrow_mut()
            .resolve(identifier.value.as_str())
        {
            Some(symbol) => symbol,
            None => return Some(format!("unknown variable. {}", identifier.value)),
        }
    };

    compiler.load_symbol(symbol);

    None
}
