use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::Compiler;
use crate::exception::compiler::{CompilerException, CompilerExceptionCode};
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::{Index, Slice};
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Compound, FunctionType, Types};

pub fn compile_expression_index(
    _compiler: &mut Compiler,
    _index: Index,
) -> Result<Types, CompilerException> {
    #[allow(clippy::single_match)]
    match _index.index.clone() {
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
) -> Result<Types, CompilerException> {
    if let Expression::Identifier(ident) = left.clone() {
        // Try to find var
        let var = _compiler.resolve_symbol(&ident.value);

        if let Some(var) = var {
            if let Types::Module(module) = var._type {
                return _compiler.compile_expression(Expression::Identifier(Identifier::new(
                    format!("{}::{}", module, field),
                    0,
                    0,
                )));
            }
        }
    }

    _compiler.drier();
    let result = _compiler.compile_expression(left.clone());
    _compiler.undrier();

    if let Ok(mut check) = result {
        if let Types::Array(_) = check {
            // Methods for arrays
            return match field.as_str() {
                "push" => Ok(Types::Function(FunctionType {
                    return_type: Box::new(Types::Void),
                    parameter_types: vec![],
                    reference: "ADD_TO_ARRAY".to_string(),
                    is_method: false,
                })),
                &_ => Err(CompilerException::new(
                    0,
                    0,
                    CompilerExceptionCode::UnknownField(field, format!("{:?}", check)),
                )),
            };
        }

        if let Types::Function(func) = check {
            check = *func.return_type;
        }

        // Check if function exists with this specific signature
        let var = _compiler.resolve_symbol(&format!("{}_{}", check.transpile(), field));

        if let Some(var) = var {
            let result = compile_expression_identifier(_compiler, Identifier::new(var.name, 0, 0));

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

    if let Ok(ref success) = result {
        let compound = find_type(success.clone(), _compiler);

        if let Some(Compound(ref name, ref fields)) = compound {
            let fields = fields.clone();
            let found = fields.iter().find(|item| item.name == field);

            return if let Some(field) = found {
                _compiler.add_to_current_function(format!(
                    "}} {{ .CONSTANT INT {}; }};",
                    (field.index as i32)
                ));

                Ok(field.class_item_type.clone())
            } else {
                Err(CompilerException::new(
                    0,
                    0,
                    CompilerExceptionCode::UnknownField(field, name.clone()),
                ))
            };
        }

        return Err(CompilerException::new(
            0,
            0,
            CompilerExceptionCode::UnknownField(field, format!("{:?}", result)),
        ));
    }

    Err(CompilerException::new(
        0,
        0,
        CompilerExceptionCode::UnknownField(field, format!("{:?}", result)),
    ))
}

pub fn compile_expression_assign_index(
    compiler: &mut Compiler,
    assign: AssignIndex,
) -> Result<Types, CompilerException> {
    compiler.add_to_current_function(".ASSIGN { ".to_string());
    if let Expression::Identifier(ident) = assign.index {
        compile_expression_class_index(compiler, assign.left, ident.value)?;

        compiler.add_to_current_function("} { ".to_string());
        compiler.compile_expression(assign.value)?;
    } else {
        compiler.add_to_current_function(".INDEX {".to_string());
        compiler.compile_expression(assign.left.clone())?;

        compiler.add_to_current_function("} {".to_string());
        compiler.compile_expression(assign.index)?;
        compiler.add_to_current_function("}; } {".to_string());

        compiler.compile_expression(assign.value)?;
    }

    compiler.add_to_current_function("};".to_string());

    Ok(Types::Void)
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> Result<Types, CompilerException> {
    compiler.add_to_current_function(".INDEX {".to_string());
    let result = compiler.compile_expression(left);
    compiler.add_to_current_function("} {".to_string());
    compiler.compile_expression(index)?;
    compiler.add_to_current_function("};".to_string());

    if let Ok(Types::Array(value_type)) = result {
        return Ok(*value_type);
    } else if let Ok(Types::Basic(BaseTypes::String)) = result {
        return Ok(Types::Basic(BaseTypes::String));
    }

    if let Ok(_type) = result {
        return Err(CompilerException::new(
            0,
            0,
            CompilerExceptionCode::WrongType(
                format!("{}", _type),
                "Expected a string or array".to_string(),
            ),
        ));
    }

    Err(CompilerException::new(0, 0, CompilerExceptionCode::Unknown))
}
/// Compiles a slice to Arc
///
/// Take this Loop code:
/// ```loop
/// var array = [10, 20, 30];
/// var sliced = array.slice(0,2);
/// ```
///
/// And generates this Arc code:
/// ```arc
/// .SLICE { .CONSTANT INT 0; } { .CONSTANT INT 2; } { .CONSTANT INT[] [10,20,30]; }
/// ```
pub fn compile_expression_slice(
    compiler: &mut Compiler,
    slice: Slice,
) -> Result<Types, CompilerException> {
    // Creating slice instruction with correct target value
    compiler.add_to_current_function(".SLICE { ".to_string());
    let result = compiler.compile_expression(*slice.left.clone());
    compiler.add_to_current_function("} { ".to_string());

    let mut slice_type = Types::Void; // Heuretics

    if let Ok(var) = result {
        // Only Arrays and strings can be slices, therefore the typecheck
        match var {
            Types::Array(_type) => slice_type = Types::Array(_type),
            Types::Basic(BaseTypes::String) => slice_type = Types::Basic(BaseTypes::String),
            _ => {
                return Err(CompilerException::new(
                    0,
                    0,
                    CompilerExceptionCode::WrongType(
                        format!("{}", var),
                        "Expected a string or array".to_string(),
                    ),
                ))
            }
        }
    };

    let start = *slice.begin.clone(); // Start of the slice
    let end = *slice.end; // End of the slice

    // FIXME: This '-1' should be done in Sanzio, not in Picasso
    compiler.compile_expression(start)?;
    compiler.add_to_current_function("} { .SUBTRACT { ".to_string());
    compiler.compile_expression(end)?;
    compiler.add_to_current_function(" .CONSTANT INT 1; }; };".to_string());

    Ok(slice_type)
}
