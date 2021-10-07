use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, EmittedInstruction};
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_return_statement(_compiler: &mut Compiler, rt: ReturnStatement) -> Option<String> {
    let err = _compiler.compile_expression(*rt.expression);

    if err.is_some() {
        return err;
    }

    _compiler.emit(OpCode::Return, vec![]);
    let pos = _compiler.emit(OpCode::Jump, vec![0]);

    _compiler.return_jumps.push(EmittedInstruction {
        position: pos as i64,
        op: OpCode::Jump,
    });

    None
}
