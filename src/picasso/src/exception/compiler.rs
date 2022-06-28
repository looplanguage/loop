use std::fmt::{Display, Formatter};
use crate::parser::exception::SyntaxException;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnknownSymbol {
    pub name: String,
    pub scope_depth: u16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompilerException {
    pub location: (i32, i32),
    pub exception: CompilerExceptionCode,
}

impl CompilerException {
    pub fn new(line: i32, colon: i32, exception: CompilerExceptionCode) -> CompilerException {
        CompilerException {
            location: (line, colon),
            exception,
        }
    }
}

impl Display for CompilerException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompilerExceptionCode {
    UnknownSymbol(UnknownSymbol),
    DivideByZero,
    TooManyLocals,
    TooManyFrees,
    UnknownSuffixOperator(String),
    ReturnStatementNotAllowedOutsideFunction,
    UnknownExtensionMethod(String),
    CanOnlyAssignToVariableArray,
    CanNotReadFile(String),
    DoubleParameterName(String),
    CallingNonFunction(String),
    /// GOT, EXPECTED
    WrongType(String, String),
    ValueDifferentFromType(String, String),
    /// Field, Type
    UnknownField(String, String),
    UnknownType(String),
    /// Module, Name
    NotPublic(String, String),
    Unknown,
}

impl From<SyntaxException> for CompilerException {
    fn from(_: SyntaxException) -> Self {
        CompilerException {
            location: (0, 0),
            exception: CompilerExceptionCode::Unknown,
        }
    }
}

impl CompilerException {
    pub fn pretty_print(&self) -> String {
        match &self.exception {
            CompilerExceptionCode::UnknownSymbol(var) => {
                format!("unknown symbol. got=\"{}\"", var.name)
            }
            CompilerExceptionCode::DivideByZero => String::from("can not divide by zero"),
            CompilerExceptionCode::TooManyLocals => String::from("too many locals"),
            CompilerExceptionCode::TooManyFrees => String::from("too many frees"),
            CompilerExceptionCode::UnknownSuffixOperator(operator) => {
                format!("unknown suffix operator. got=\"{}\"", operator)
            }
            CompilerExceptionCode::ReturnStatementNotAllowedOutsideFunction => {
                String::from("return statements are not allowed outside of functions")
            }
            CompilerExceptionCode::UnknownExtensionMethod(method) => {
                format!("unknown extension method. got=\"{}\"", method)
            }
            CompilerExceptionCode::CanOnlyAssignToVariableArray => {
                String::from("you can only assign to variable arrays")
            }
            CompilerExceptionCode::CanNotReadFile(error) => {
                format!("unable to read file. got=\"{}\"", error)
            }
            CompilerExceptionCode::DoubleParameterName(param) => {
                format!("parameter name already in use. got=\"{}\"", param)
            }
            CompilerExceptionCode::CallingNonFunction(f) => {
                format!("you are attempting to call a non function. got=\"{}\". expected=\"Function(...)\"", f)
            }
            CompilerExceptionCode::WrongType(got, expected) => {
                format!(
                    "type mismatch, can not assign different type. got=\"{}\". expected=\"{}\"",
                    got, expected
                )
            }
            CompilerExceptionCode::ValueDifferentFromType(got, expected) => {
                format!(
                    "type mismatch, can not declare variable with static type to different typed value. got=\"{}\". expected\"{}\"",
                    got, expected
                )
            }
            CompilerExceptionCode::UnknownField(field, class) => {
                format!(
                    "field does not exist on type. field=\"{}\". type=\"{}\"",
                    field, class
                )
            }
            CompilerExceptionCode::UnknownType(tp) => {
                format!("type does not exist. got=\"{}\"", tp)
            }
            CompilerExceptionCode::Unknown => "got an error, unknown what went wrong".to_string(),
            CompilerExceptionCode::NotPublic(module, name) => format!(
                "Method \"{}\" inside module \"{}\" is not public!",
                name, module
            ),
        }
    }

    pub fn emit(&self) {
        /*
        match self {
            CompilerException::UnknownSymbol(symbol) => {
                sentry::with_scope(
                    |scope| {
                        scope.set_tag("exception.type", "compiler");

                        let mut map: BTreeMap<String, Value> = std::collections::BTreeMap::new();
                        map.insert(String::from("variable_name"), symbol.name.clone().into());
                        map.insert(String::from("scope_depth"), symbol.scope_depth.into());

                        scope.set_context("", sentry::protocol::Context::Other(map));
                    },
                    || {
                        sentry::capture_message("UnknownSymbol", sentry::Level::Info);
                    },
                );
            }
            _ => {
                sentry::with_scope(
                    |scope| {
                        scope.set_tag("exception.type", "compiler");
                    },
                    || match self {
                        CompilerException::UnknownSuffixOperator(suffix) => {
                            sentry::capture_message(
                                format!("UnknownSuffixOperator: {}", suffix).as_str(),
                                sentry::Level::Info,
                            );
                        }
                        _ => {
                            sentry::capture_message(
                                format!("{:?}", self).as_str(),
                                sentry::Level::Info,
                            );
                        }
                    },
                );
            }
        }*/
    }
}
