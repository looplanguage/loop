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

    #[allow(clippy::single_match)]
    match _index.index.clone() {
        Expression::Call(call) => {
            compile_expression_extension_method(_compiler, call, _index.left)
        }
        _ => compile_expression_index_internal(_compiler, _index.left, _index.index),
    }
}

pub fn compile_expression_assign_index(
    compiler: &mut Compiler,
    assign: AssignIndex,
) -> CompilerResult {
    compiler.compile_expression(assign.left);

    compiler.add_to_current_function("[".to_string());
    compiler.compile_expression(assign.index);
    compiler.add_to_current_function("] = ".to_string());
    compiler.compile_expression(assign.value);

    CompilerResult::Success
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> CompilerResult {
    compiler.compile_expression(left);
    compiler.add_to_current_function("[".to_string());
    compiler.compile_expression(index);
    compiler.add_to_current_function("]".to_string());

    compiler.emit(OpCode::Index, vec![]);

    CompilerResult::Success
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
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

    compiler.compile_expression(left.clone());

    match method_id.unwrap() {
        2 => transpile_extension_add(compiler, call),
        3 => transpile_extension_remove(compiler, call, left),
        4 => transpile_extension_slice(compiler, call),
        5 => transpile_extension_length(compiler),
        _ => CompilerResult::Exception(CompilerException::Unknown),
    }
}

fn transpile_extension_add(compiler: &mut Compiler, call: Call) -> CompilerResult {
    /// Transpiles the extension method 'add'
    ///
    /// Take this Loop code:
    /// ```loop
    /// var array = [10, 20, 30];
    /// array.add(40, 50);
    /// ```
    ///
    /// And generates this D code:
    /// ```d
    /// auto var_array_0 = [10, 20, 30];
    /// var_array_0 ~= [40, 50];
    /// ```
    compiler.add_to_current_function(" ~= ".to_string());

    let mut index = 0;

    if call.parameters.len() > 1 {
        compiler.add_to_current_function("[".to_string());
    }

    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }

        index += 1;

        if call.parameters.len() > 1 && call.parameters.len() != index {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    if call.parameters.len() > 1 {
        compiler.add_to_current_function("]".to_string());
    }

    CompilerResult::Success
}

fn transpile_extension_remove(compiler: &mut Compiler, call: Call, left: Expression) -> CompilerResult {
    /// Transpiles the extension method 'add'
    ///
    /// Take this Loop code:
    /// ```loop
    /// var array = [10, 20, 30];
    /// array.remove(0, 1);
    /// ```
    ///
    /// And generates this D code:
    /// ```d
    /// auto var_array_0 = [10, 20, 30];
    /// var_array_0 = var_array_0.remove(0, 1);
    /// ```
    compiler.add_to_current_function(" = ".to_string());

    compiler.compile_expression(left);

    compiler.add_to_current_function(".remove(".to_string());

    let mut index = 0;
    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }

        index += 1;

        if call.parameters.len() > 1 && call.parameters.len() != index {
            compiler.add_to_current_function(", ".to_string());
        }
    }

    compiler.add_to_current_function(")".to_string());

    CompilerResult::Success
}

fn transpile_extension_slice(compiler: &mut Compiler, call: Call) -> CompilerResult {
    /// Transpiles the extension method 'add'
    ///
    /// Take this Loop code:
    /// ```loop
    /// var array = [10, 20, 30];
    /// var sliced = array.slice(0,2);
    /// ```
    ///
    /// And generates this D code:
    /// ```d
    /// auto var_array_0 = [10, 20, 30];
    /// auto sliced = var_array_0[0..2];
    /// ```
    compiler.add_to_current_function("[".to_string());

    let start = call.parameters[0].clone();
    let end = call.parameters[1].clone();

    compiler.compile_expression(start);

    compiler.add_to_current_function("..".to_string());

    compiler.compile_expression(end);

    compiler.add_to_current_function("]".to_string());

    CompilerResult::Success
}

fn transpile_extension_length(compiler: &mut Compiler) -> CompilerResult {
    /// Transpiles the extension method 'add'
    ///
    /// Take this Loop code:
    /// ```loop
    /// var array = [10, 20, 30];
    /// var length = array.length()
    /// ```
    ///
    /// And generates this D code:
    /// ```d
    /// auto var_array_0 = [10, 20, 30];
    /// auto length = var_array_0.length;
    /// ```
    compiler.add_to_current_function(".length".to_string());

    CompilerResult::Success
}
