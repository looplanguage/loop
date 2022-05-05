use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_index(_compiler: &mut Compiler, _index: Index) -> CompilerResult {
    // Change to a match when indexing with [] (eg array[0])

    #[allow(clippy::single_match)]
    match _index.index.clone() {
        Expression::Call(call) => compile_expression_extension_method(_compiler, call, _index.left),
        _ => compile_expression_index_internal(_compiler, _index.left, _index.index),
    }
}

fn _get_array_value_type(result: CompilerResult) -> Types {
    if let CompilerResult::Success(Types::Array(value_type)) = result {
        return *value_type;
    }

    Types::Auto
}

pub fn compile_expression_assign_index(
    compiler: &mut Compiler,
    assign: AssignIndex,
) -> CompilerResult {
    let var = compiler.define_variable("ptr_to_array".to_string(), Types::Auto);

    compiler.add_to_current_function(format!("auto {} = ", var.transpile()));
    compiler.compile_expression(assign.left, false);

    compiler.add_to_current_function(format!("; {}[", var.transpile()));
    compiler.compile_expression(assign.index, false);
    compiler.add_to_current_function("] = ".to_string());
    compiler.compile_expression(assign.value, false);

    CompilerResult::Success(Types::Void)
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> CompilerResult {
    let result = compiler.compile_expression(left, false);
    compiler.add_to_current_function("[".to_string());
    compiler.compile_expression(index, false);
    compiler.add_to_current_function("]".to_string());

    if let CompilerResult::Success(Types::Array(value_type)) = result {
        return CompilerResult::Success(*value_type);
    }

    // TODO: Proper error
    CompilerResult::Exception(CompilerException::Unknown)
}

/*
   extension!(to_string),
   extension!(to_int),
   extension!(add),
   extension!(remove),
   extension!(slice),
   extension!(length),
*/

const EXTENSION_METHODS: &[&str] = &["to_string", "to_int", "add", "remove", "slice", "length"];

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
) -> CompilerResult {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Check if method exists in a library based on the "left".

    // Search extension id
    let method_id = EXTENSION_METHODS.iter().position(|&m| m == method.as_str());

    if method_id.is_none() {
        return CompilerResult::Exception(CompilerException::UnknownExtensionMethod(method));
    }

    match method_id.unwrap() {
        0 => transpile_extension_to_string(compiler, left),
        1 => transpile_extension_to_int(compiler, left),
        2 => transpile_extension_add(compiler, call, left),
        3 => transpile_extension_remove(compiler, call, left),
        4 => transpile_extension_slice(compiler, call, left),
        5 => transpile_extension_length(compiler, left),
        _ => CompilerResult::Exception(CompilerException::Unknown),
    }
}

/// Transpiles the extension method 'to_string'
///
/// Take this Loop code:
/// ```loop
/// 500.to_string()
/// ```
///
/// And generates this D code:
/// ```d
/// to!string(500)
/// ```
fn transpile_extension_to_string(compiler: &mut Compiler, left: Expression) -> CompilerResult {
    let var = compiler.define_variable("tmp_to_convert".to_string(), Types::Auto);

    compiler.add_to_current_function(format!("() {{ auto {} = ", var.transpile()));

    let result = compiler.compile_expression(left, false);

    compiler.add_to_current_function(";".to_string());

    if let CompilerResult::Exception(exception) = result {
        return CompilerResult::Exception(exception);
    }

    compiler.add_to_current_function(format!("return to!string({}); }}()", var.transpile()));

    CompilerResult::Success(Types::Basic(BaseTypes::String))
}

/// Transpiles the extension method 'to_int'
///
/// Take this Loop code:
/// ```loop
/// "500".to_int()
/// ```
///
/// And generates this D code:
/// ```d
/// to!int("500")
/// ```
fn transpile_extension_to_int(compiler: &mut Compiler, left: Expression) -> CompilerResult {
    let var = compiler.define_variable("tmp_to_convert".to_string(), Types::Auto);

    compiler.add_to_current_function(format!("() {{ auto {} = ", var.transpile()));

    let result = compiler.compile_expression(left, false);

    compiler.add_to_current_function(".to!string;".to_string());

    if let CompilerResult::Exception(exception) = result {
        return CompilerResult::Exception(exception);
    }

    compiler.add_to_current_function(format!("return to!int({}); }}()", var.transpile()));

    CompilerResult::Success(Types::Basic(BaseTypes::Integer))
}

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
fn transpile_extension_add(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
) -> CompilerResult {
    compiler.add_to_current_function(".PUSH { ".to_string());
    let result = compiler.compile_expression(left, false);

    compiler.replace_at_current_function("{INSERT}".to_string(), ".STORE {".to_string());

    compiler.add_to_current_function("} { ".to_string());

    let mut index = 0;

    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter, false);

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

    compiler.add_to_current_function("};".to_string());

    result
}

/// Transpiles the extension method 'remove'
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
fn transpile_extension_remove(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
) -> CompilerResult {
    compiler.add_to_current_function(".SLICE { ".to_string());

    let mut index = 0;
    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter, false);

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

    compiler.add_to_current_function("} { .ADD { ".to_string());

    let mut index = 0;
    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter, false);

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


    compiler.add_to_current_function(" .CONSTANT INT 1;}; } {".to_string());

    let result = compiler.compile_expression(left.clone(), false);
    compiler.add_to_current_function("}".to_string());

    result
}

/// Transpiles the extension method 'slice'
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
fn transpile_extension_slice(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
) -> CompilerResult {
    let result = compiler.compile_expression(left, false);

    let mut slice_type = Types::Void;

    if let CompilerResult::Success(array_type) = result {
        if let Types::Array(_type) = array_type {
            slice_type = Types::Array(_type);
        } else {
            // TODO: Return better error
            return CompilerResult::Exception(CompilerException::Unknown);
        }
    };

    // Maybe add .get!(Variant[]) if auto?
    compiler.add_to_current_function("[".to_string());

    let start = call.parameters[0].clone();
    let end = call.parameters[1].clone();

    compiler.compile_expression(start, false);

    compiler.add_to_current_function("..".to_string());

    compiler.compile_expression(end, false);

    compiler.add_to_current_function("]".to_string());

    CompilerResult::Success(slice_type)
}

/// Transpiles the extension method 'length'
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
/// auto length = to!int(var_array_0.length);
/// ```
fn transpile_extension_length(compiler: &mut Compiler, left: Expression) -> CompilerResult {
    compiler.add_to_current_function("to!int(".to_string());

    compiler.compile_expression(left, false);

    compiler.add_to_current_function(".length)".to_string());

    CompilerResult::Success(Types::Basic(BaseTypes::Integer))
}
