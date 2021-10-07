use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::conditional::Conditional;

pub fn compile_expression_conditional(
    compiler: &mut Compiler,
    conditional: Conditional,
) -> Option<String> {
    let err = compiler.compile_expression(*conditional.condition);

    if err.is_some() {
        return err;
    }

    let position = compiler.emit(OpCode::JumpIfFalse, vec![0]);

    let err = compiler.compile_block(conditional.body);

    if err.is_some() {
        return err;
    }

    if !compiler.remove_last(OpCode::Pop) {
        return Some("unable to remove last pop".to_string());
    }

    let len = compiler.instructions.len() as u32;
    compiler.change_operand(position as u32, vec![len]);

    None
}
