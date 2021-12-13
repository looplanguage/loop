use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::function::Call;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    let err = compiler.compile_expression(*call.identifier.clone());

    if err.is_some() {
        return CompilerResult::Exception(err.unwrap());
    }

    for parameter in call.parameters.clone() {
        let err = compiler.compile_expression(parameter);
        if err.is_some() {
            return CompilerResult::Exception(err.unwrap());
        }
    }

    let param_len = call.parameters.len();

    compiler.emit(OpCode::Call, vec![param_len as u32]);

    CompilerResult::Success
}
