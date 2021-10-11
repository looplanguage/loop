use crate::compiler::opcode::OpCode;
use crate::compiler::variable::Scope;
use crate::compiler::Compiler;
use crate::compiler::symbol_table::Scope;
use crate::parser::expression::identifier::Identifier;

pub fn compile_expression_identifier(
    compiler: &mut Compiler,
    identifier: Identifier,
) -> Option<String> {
    let var = compiler
        .symbol_table
        .find_variable(identifier.value.clone());

    if var.is_none() {
        return Some(format!(
            "variable \"{}\" is not defined in this scope",
            identifier.value
        ));
    }

    let unwrapped = var.unwrap();

    if unwrapped.scope == Scope::Global {
        compiler.emit(OpCode::GetVar, vec![unwrapped.index]);
    } else if unwrapped.scope == Scope::Local {
        compiler.emit(OpCode::GetLocal, vec![unwrapped.index]);
    } else {
        compiler.emit(OpCode::GetFree, vec![unwrapped.index]);
    }

    None
}
