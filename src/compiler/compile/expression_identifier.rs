use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> Option<String> {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(identifier.value.as_str());

    if symbol.is_none() {
        let var = compiler
            .variable_scope
            .borrow_mut()
            .resolve(identifier.value.clone());

        if var.is_some() {
            compiler.emit(OpCode::GetVar, vec![var.unwrap().index]);
            return None;
        }
    } else {
        compiler.load_symbol(symbol.unwrap());
        return None;
    }

    Some(format!("unknown variable. got=\"{}\"", identifier.value))
}
