use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
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
    };

    None
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

    // Search for extension method based on type
    let method_id = match left.clone() {
        Expression::Integer(integer) => {
            let extension = integer.find_extension(method.as_str());

            if let Some(extension) = extension {
                compiler.last_extension_type = Option::from(extension.1);
                Some(extension.0)
            } else {
                None
            }
        }
        Expression::String(string) => {
            let extension = string.find_extension(method.as_str());

            if let Some(extension) = extension {
                compiler.last_extension_type = Option::from(extension.1);
                Some(extension.0)
            } else {
                None
            }
        }
        Expression::Index(index) => {
            compile_expression_index(compiler, *index);

            let last_extension_type = compiler.last_extension_type.clone().unwrap();

            return compile_expression_extension_method(compiler, call, last_extension_type, false);
        }
        Expression::Identifier(identifier) => {
            compile_expression_identifier(compiler, identifier.clone());

            let var = compiler.variable_scope.as_ref().borrow_mut().resolve(identifier.value);

            return if let Some(var) = var {
                compile_expression_extension_method(compiler, call, var._type, false)
            } else {
                Some(CompilerException::UnknownExtensionMethod(method))
            }
        }
        _ => return Some(CompilerException::UnknownExtensionMethod(method)),
    };

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
