use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::extension_method::lookup;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;

pub fn compile_expression_index(_compiler: &mut Compiler, _index: Index) -> CompilerResult {
    // Change to a match when indexing with [] (eg array[0])

    match _index.index.clone() {
        Expression::Call(call) => {
            compile_expression_extension_method(_compiler, call, _index.left, true)
        }
        _ => compile_expression_index_internal(_compiler, _index.left, _index.index),
    }
}

pub fn compile_expression_assign_index(
    compiler: &mut Compiler,
    assign: AssignIndex,
) -> CompilerResult {
    compiler.compile_expression(assign.left);
    compiler.compile_expression(assign.index);
    compiler.compile_expression(assign.value);

    compiler.emit(OpCode::AssignIndex, vec![]);

    CompilerResult::Success
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> CompilerResult {
    compiler.compile_expression(left);
    compiler.compile_expression(index);

    compiler.emit(OpCode::Index, vec![]);

    CompilerResult::Success
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
    deeper: bool,
) -> CompilerResult {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Search extension id
    let method_id = lookup(method.as_str());

    if method_id.is_none() {
        return CompilerResult::Exception(CompilerException::UnknownExtensionMethod(method));
    }

    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter);

        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }
    }

    let param_len = call.parameters.len();

    if deeper {
        compiler.compile_expression(left);
    }

    compiler.emit(
        OpCode::CallExtension,
        vec![method_id.unwrap() as u32, param_len as u32],
    );

    CompilerResult::Success
}
