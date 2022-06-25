use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::{Index, Slice};
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Compound, Types};

pub fn compile_expression_index(_compiler: &mut Compiler, _index: Index) -> CompilerResult {
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
) -> CompilerResult {
    _compiler.drier();
    let result = _compiler.compile_expression(left.clone());
    _compiler.undrier();

    if let CompilerResult::Success(mut check) = result {
        if let Types::Function(func) = check {
            check = *func.return_type;
        }

        // Check if function exists with this specific signature
        let var = _compiler.variable_scope.borrow_mut().resolve(format!(
            "{}_{}",
            check.transpile(),
            field
        ));

        if let Some(var) = var {
            let result = compile_expression_identifier(_compiler, Identifier { value: var.name });

            return result;
        }
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
                let var = _compiler.variable_scope.borrow_mut().resolve(user.clone());

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
        compiler.compile_expression(assign.left);

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
    } else if let CompilerResult::Success(Types::Basic(BaseTypes::String)) = result {
        return CompilerResult::Success(Types::Basic(BaseTypes::String));
    }

    if let CompilerResult::Success(_type) = result {
        return CompilerResult::Exception(CompilerException::WrongType(
            format!("{}", _type),
            "Expected a string or array".to_string(),
        ));
    }
    CompilerResult::Exception(CompilerException::Unknown)
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
pub fn compile_expression_slice(compiler: &mut Compiler, slice: Slice) -> CompilerResult {
    compiler.add_to_current_function(".SLICE { ".to_string());
    let result = compiler.compile_expression(*slice.left.clone());
    compiler.add_to_current_function("} { ".to_string());

    let mut slice_type = Types::Void;

    if let CompilerResult::Success(var) = result {
        match var {
            Types::Array(_type) => slice_type = Types::Array(_type),
            Types::Basic(BaseTypes::String) => slice_type = Types::Basic(BaseTypes::String),
            _ => return CompilerResult::Exception(CompilerException::Unknown),
        }
    };

    let start = *slice.begin.clone();
    let end = *slice.end;

    compiler.compile_expression(start);

    compiler.add_to_current_function("} { .SUBTRACT { ".to_string());

    compiler.compile_expression(end);

    compiler.add_to_current_function(" .CONSTANT INT 1; }; };".to_string());

    CompilerResult::Success(slice_type)
}
