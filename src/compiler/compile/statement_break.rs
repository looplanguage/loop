use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::break_statement::BreakStatement;

pub fn compile_break_statement(
    _compiler: &mut Compiler,
    _br: BreakStatement,
) -> Option<CompilerException> {
    /*
    let err = _compiler.compile_expression(*rt.expression);
    if err.is_some() {
        return err;
    }*/

    let jump = _compiler.emit(OpCode::Jump, vec![99999]);

    _compiler.breaks.push(jump as u32);

    None
}
