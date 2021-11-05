use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::extension_method::lookup;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;

pub fn compile_expression_index(
    _compiler: &mut Compiler,
    _index: Index,
) -> Option<CompilerException> {
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
) -> Option<CompilerException> {
    compiler.compile_expression(assign.left);
    compiler.compile_expression(assign.index);
    compiler.compile_expression(assign.value);

    compiler.emit(OpCode::AssignIndex, vec![]);

    None
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> Option<CompilerException> {
    compiler.compile_expression(left);
    compiler.compile_expression(index);

    compiler.emit(OpCode::Index, vec![]);

    None
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
    deeper: bool,
) -> Option<CompilerException> {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Search extension id
    let method_id = lookup(method.as_str());

    if method_id.is_none() {
        return Some(CompilerException::UnknownExtensionMethod(method));
    }

    for parameter in call.parameters.clone() {
        let err = compiler.compile_expression(parameter);
        if err.is_some() {
            return err;
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

    None
}
