use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::conditional::Conditional;
use crate::parser::program::Node;

pub fn compile_expression_conditional(
    compiler: &mut Compiler,
    conditional: Conditional,
) -> Option<String> {
    let err = compiler.compile_expression(*conditional.condition);

    if err.is_some() {
        return err;
    }

    let position_false = compiler.emit(OpCode::JumpIfFalse, vec![0]);

    let err = compiler.compile_block(conditional.body);

    if err.is_some() {
        return err;
    }

    compiler.remove_last(OpCode::Pop);

    let position = compiler.emit(OpCode::Jump, vec![0]);

    let len = compiler.instructions.len() as u32;
    compiler.change_operand(position_false as u32, vec![len]);

    compile_expression_null(compiler);

    let len = compiler.instructions.len() as u32;
    compiler.change_operand(position as u32, vec![len]);

    if let Some(node) = conditional.else_condition.as_ref() {
        if let Node::Expression(exp) = node {
            compiler.compile_expression(exp.clone());
        }
    }

    if compiler.current_variable_scope.outer.is_none() {
        for return_jump in compiler.return_jumps.clone() {
            let pos = return_jump.position;
            compiler.change_operand(pos as u32, vec![len]);
        }

        compiler.return_jumps = vec![];
    }

    None
}
