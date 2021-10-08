use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, EmittedInstruction};
use crate::parser::statement::return_statement::ReturnStatement;

pub fn compile_return_statement(_compiler: &mut Compiler, rt: ReturnStatement) -> Option<String> {
    if _compiler.scope_index == 0 {
        return Some("return statement not allowed outside of functions".to_string());
    }

    let err = _compiler.compile_expression(*rt.expression);

    if err.is_some() {
        return err;
    }

    _compiler.emit(OpCode::Return, vec![]);

    None
}
