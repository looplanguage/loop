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
    if let Expression::Identifier(ident) = left.clone() {
        // Try to find var
        let var = _compiler.resolve_symbol(&ident.value);

        if let Some(var) = var {
            if let Types::Module(module) = var._type {
                return _compiler.compile_expression(Expression::Identifier(Identifier {
                    value: format!("{}::{}", module, field),
                }));
            }
        }
    }

    _compiler.drier();
    let result = _compiler.compile_expression(left.clone());
    _compiler.undrier();

    if let CompilerResult::Success(mut check) = result {
        if let Types::Array(_) = check {
            // Methods for arrays
            return match field.as_str() {
                "push" => CompilerResult::Success(Types::Function(FunctionType {
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
        let var = _compiler.resolve_symbol(&format!("{}_{}", check.transpile(), field));

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
                let var = _compiler.resolve_symbol(user);

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
    if let Expression::Identifier(ident) = assign.index {
        compile_expression_class_index(compiler, assign.left, ident.value);

        compiler.add_to_current_function("} { ".to_string());
        compiler.compile_expression(assign.value);
    } else {
        compiler.add_to_current_function(".INDEX {".to_string());
        compiler.compile_expression(assign.left.clone());

        compiler.add_to_current_function("} {".to_string());
        compiler.compile_expression(assign.index);
        compiler.add_to_current_function("}; } {".to_string());

        compiler.compile_expression(assign.value);
    }

    compiler.add_to_current_function("};".to_string());

    CompilerResult::Success(Types::Void)
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
    if let Expression::Identifier(ident) = left {
        let var = compiler.resolve_symbol(&ident.value);

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

    CompilerResult::Exception(CompilerException::Unknown)
}
