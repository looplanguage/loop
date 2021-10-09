use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> Option<String> {
    let var = compiler
        .current_variable_scope
        .find_variable(identifier.value.clone());

    // TODO: Recursive functions aren't defined inside their own functions
    if var.is_none() {
        return Some(format!(
            "variable \"{}\" is not defined in this scope",
            identifier.value
        ));
    }
    println!("OpGetVar {}", identifier.value.clone());

    let unwrapped = var.unwrap().index;

    compiler.emit(OpCode::GetVar, vec![unwrapped]);

    None
}
