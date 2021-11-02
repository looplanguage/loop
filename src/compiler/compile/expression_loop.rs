use crate::compiler::Compiler;
use crate::compiler::opcode::OpCode;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::loops::Loop;

pub fn compile_loop_expression(
    compiler: &mut Compiler,
    lp: Loop,
) -> Option<CompilerException> {
    let start = compiler.scope().instructions.len();
    let err = compiler.compile_expression(*lp.condition);

    if err.is_some() {
        return err;
    }

    let done = compiler.emit(OpCode::JumpIfFalse, vec![99999]); // To jump later

    let err = compiler.compile_block(lp.body);

    if err.is_some() {
        return err;
    }

    compiler.emit(OpCode::Jump, vec![start as u32]); // Jump back to start

    compiler.change_operand(done as u32, vec![compiler.scope().instructions.len() as u32]);

    compiler.emit(OpCode::Constant, vec![0]);

    None
}