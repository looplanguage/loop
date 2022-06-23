use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Compound, FunctionType, Types};

pub fn compile_expression_index(_compiler: &mut Compiler, _index: Index) -> CompilerResult {
    // Change to a match when indexing with [] (eg array[0])

    #[allow(clippy::single_match)]
    match _index.index.clone() {
        Expression::Call(call) => compile_expression_extension_method(_compiler, call, _index.left),
        Expression::Identifier(ident) => {
            compile_expression_class_index(_compiler, _index.left, ident.value)
        }
        _ => compile_expression_index_internal(_compiler, _index.left, _index.index),
    }
}

fn compile_expression_class_index(
    _compiler: &mut Compiler,
    left: Expression,
    field: String,
) -> CompilerResult {
    _compiler.drier();
    let result = _compiler.compile_expression(left.clone());
    _compiler.undrier();

    if let CompilerResult::Success(mut check) = result {
        if let Types::Array(_) = check {
            // Methods for arrays
            return match field.as_str() {
                "add" => CompilerResult::Success(Types::Function(FunctionType {
                    return_type: Box::new(Types::Void),
                    parameter_types: vec![],
                    reference: "ADD_TO_ARRAY".to_string(),
                    is_method: false,
                })),
                &_ => CompilerResult::Exception(CompilerException::UnknownField(
                    field,
                    format!("{:?}", check),
                )),
            };
        }

        if let Types::Function(func) = check {
            check = *func.return_type;
        }

        // Check if function exists with this specific signature
        let var = _compiler.resolve_variable(&format!("{}{}", check.transpile(), field));

        if let Some(var) = var {
            let result = compile_expression_identifier(_compiler, Identifier { value: var.name });

            return result;
        }
    } else {
        return result;
    }

    _compiler.add_to_current_function(".INDEX { ".to_string());

    let result = _compiler.compile_expression(left);

    fn find_type(_type: Types, _compiler: &mut Compiler) -> Option<Compound> {
        match _type {
            Types::Compound(c) => Some(c),
            Types::Function(func) => match *func.return_type {
                Types::Function(f) => find_type(*f.return_type, _compiler),
                Types::Compound(c) => Some(c),
                _ => None,
            },
            Types::Basic(BaseTypes::UserDefined(ref user)) => {
                // Find a user defined type
                let var = _compiler.resolve_variable(user);

                if let Some(var) = var {
                    if let Types::Compound(c) = var._type {
                        return Some(c);
                    }
                }

                None
            }
            _ => None,
        }
    }

    if let CompilerResult::Success(ref success) = result {
        let compound = find_type(success.clone(), _compiler);

        if let Some(Compound(ref name, ref fields)) = compound {
            let fields = fields.clone();
            let found = fields.iter().find(|item| item.name == field);

            if let Some(field) = found {
                _compiler.add_to_current_function(format!(
                    "}} {{ .CONSTANT INT {}; }};",
                    (field.index as i32)
                ));

                return CompilerResult::Success(field.class_item_type.clone());
            } else {
                return CompilerResult::Exception(CompilerException::UnknownField(
                    field,
                    name.clone(),
                ));
            }
        }

        return CompilerResult::Exception(CompilerException::UnknownField(
            field,
            format!("{:?}", result),
        ));
    }

    CompilerResult::Exception(CompilerException::UnknownField(
        field,
        format!("{:?}", result),
    ))
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
    compiler.add_to_current_function(".ASSIGN { ".to_string());
    let mut err = None;
    if let Expression::Identifier(ident) = assign.index {
        err = Some(compile_expression_class_index(compiler, assign.left, ident.value));

        compiler.add_to_current_function("} { ".to_string());
        println!("VALUE: {:?}", assign.value.clone());
        err = Some(compiler.compile_expression(assign.value));
    } else {
        compiler.add_to_current_function(".INDEX {".to_string());
        err = Some(compiler.compile_expression(assign.left.clone()));
        println!("ERROR: {:?}", err);

        compiler.add_to_current_function("} {".to_string());
        err = Some(compiler.compile_expression(assign.index));
        println!("ERROR: {:?}", err);
        compiler.add_to_current_function("}; } {".to_string());

        err = Some(compiler.compile_expression(assign.value));
    }

    compiler.add_to_current_function("};".to_string());

    err.unwrap_or(CompilerResult::Success(Types::Void))
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> CompilerResult {
    compiler.add_to_current_function(".INDEX {".to_string());
    let result = compiler.compile_expression(left);
    compiler.add_to_current_function("} {".to_string());
    compiler.compile_expression(index);
    compiler.add_to_current_function("};".to_string());

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
    if let Expression::Identifier(ident) = left.clone() {
        let var = compiler.resolve_variable(&ident.value);

        if let Some(var) = var {
            match var._type {
                Types::Library(lib) => {
                    if lib.methods.contains(&method) {
                        compiler
                            .add_to_current_function(format!(".CALL {}::{} {{", var.name, method));

                        for parameter in call.parameters {
                            let result = compiler.compile_expression(parameter);

                            #[allow(clippy::single_match)]
                            match &result {
                                CompilerResult::Exception(_exception) => return result,
                                _ => (),
                            }
                        }

                        compiler.add_to_current_function("};".to_string());

                        // Should return what the library says it should return
                        return CompilerResult::Success(Types::Void);
                    }
                }
                Types::Compound(Compound(_, fields)) => {
                    println!("FIELDS: {:?}", fields);
                }
                _ => (),
            }
        }
    }

    // Search extension id
    let method_id = EXTENSION_METHODS.iter().position(|&m| m == method.as_str());

    if method_id.is_none() {
        return CompilerResult::Success(Types::Void);
    }

    match method_id.unwrap() {
        2 => transpile_extension_add(compiler, call, left),
        3 => transpile_extension_remove(compiler, call, left),
        4 => transpile_extension_slice(compiler, call, left),
        _ => unreachable!("Should not be here"),
    }
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
    for parameter in call.parameters {
        compiler.add_to_current_function(".PUSH { ".to_string());
        compiler.compile_expression(left.clone());

        compiler.add_to_current_function("} { ".to_string());

        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }

        compiler.add_to_current_function("};".to_string());
    }

    CompilerResult::Success(Types::Void)
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
    for parameter in call.parameters {
        compiler.add_to_current_function(".POP { ".to_string());
        compiler.compile_expression(left.clone());

        compiler.add_to_current_function("} { ".to_string());

        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }

        compiler.add_to_current_function("};".to_string());
    }

    CompilerResult::Success(Types::Void)
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
    compiler.add_to_current_function(".SLICE { ".to_string());
    let result = compiler.compile_expression(left);
    compiler.add_to_current_function("} { ".to_string());

    let mut slice_type = Types::Void;

    if let CompilerResult::Success(array_type) = result {
        if let Types::Array(_type) = array_type {
            slice_type = Types::Array(_type);
        } else {
            // TODO: Return better error
            return CompilerResult::Exception(CompilerException::Unknown);
        }
    };

    let start = call.parameters[0].clone();
    let end = call.parameters[1].clone();

    compiler.compile_expression(start);

    compiler.add_to_current_function("} { .SUBTRACT { ".to_string());

    compiler.compile_expression(end);

    compiler.add_to_current_function(" .CONSTANT INT 1; }; };".to_string());

    CompilerResult::Success(slice_type)
}
