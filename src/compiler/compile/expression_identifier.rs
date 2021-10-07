use crate::compiler::Compiler;
use crate::compiler::opcode::OpCode;
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(compiler: &mut Compiler, identifier: Identifier) -> Option<String> {
    let var = compiler
        .current_variable_scope
        .find_variable(identifier.value.clone());

    if var.is_none() {
        return Some(format!(
            "variable \"{}\" is not defined in this scope",
            identifier.value
        ));
    }

    let unwrapped = var.unwrap().index;

    compiler.emit(OpCode::GetVar, vec![unwrapped]);

    None
}