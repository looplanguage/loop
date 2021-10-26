use sentry::protocol::Value;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct UnknownSymbol {
    pub name: String,
    pub scope_depth: u16,
}

#[derive(Debug, PartialEq)]
pub enum CompilerException {
    UnknownSymbol(UnknownSymbol),
    DivideByZero,
    TooManyLocals,
    TooManyFrees,
    UnknownSuffixOperator(String),
    ReturnStatementNotAllowedOutsideFunction,
    UnknownExtensionMethod(String),
    CanOnlyAssignToVariableArray,
}

impl CompilerException {
    pub fn pretty_print(&self) -> String {
        match self {
            CompilerException::UnknownSymbol(var) => {
                format!("unknown symbol. got=\"{}\"", var.name)
            }
            CompilerException::DivideByZero => String::from("can not divide by zero"),
            CompilerException::TooManyLocals => String::from("too many locals"),
            CompilerException::TooManyFrees => String::from("too many frees"),
            CompilerException::UnknownSuffixOperator(operator) => {
                format!("unknown suffix operator. got=\"{}\"", operator)
            }
            CompilerException::ReturnStatementNotAllowedOutsideFunction => {
                String::from("return statements are not allowed outside of functions")
            }
            CompilerException::UnknownExtensionMethod(method) => {
                format!("unknown extension method. got=\"{}\"", method)
            }
            CompilerException::CanOnlyAssignToVariableArray => {
                String::from("you can only assign to variable arrays")
            }
        }
    }

    pub fn emit(&self) {
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
        }
    }
}
