use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::conditional::Conditional;
use crate::parser::program::Node;
use crate::parser::statement::Statement;

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

    // TODO: Again, improve this monstrosity somehow
    if !compiler.last_is(OpCode::Return)
        && !compiler.last_is(OpCode::Constant)
        && !compiler.last_is(OpCode::GetVar)
        && !compiler.last_is(OpCode::Multiply)
        && !compiler.last_is(OpCode::Add)
        && !compiler.last_is(OpCode::Minus)
        && !compiler.last_is(OpCode::Divide)
        && !compiler.last_is(OpCode::Modulo)
    {
        compiler.emit(OpCode::Constant, vec![0]); // adds NULL to the stack
    }

    let jump_to_end = compiler.emit(OpCode::Jump, vec![0]);

    if conditional.else_condition.is_none() {
        let len = compiler.scope().instructions.len() as u32;
        compiler.change_operand(position_false as u32, vec![len]);
    }

    compile_expression_null(compiler);

    if conditional.else_condition.is_some() {
        let len = compiler.scope().instructions.len() as u32;
        compiler.change_operand(position_false as u32, vec![len]);
    }

    if let Some(node) = conditional.else_condition.as_ref() {
        if let Node::Expression(exp) = node {
            compiler.compile_expression(exp.clone());
        }
        if let Node::Statement(stmt) = node {
            if let Statement::Block(block) = stmt.clone() {
                compiler.compile_block(block);

                compiler.remove_last(OpCode::Pop);
            }
        }
    }

    let len = compiler.scope().instructions.len() as u32;
    compiler.change_operand(jump_to_end as u32, vec![len]);

    None
}
