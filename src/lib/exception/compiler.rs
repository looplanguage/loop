#[derive(Debug, PartialEq, Clone)]
pub struct UnknownSymbol {
    pub name: String,
    pub scope_depth: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompilerException {
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
    WrongType(String, String),
    Unknown,
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
            CompilerException::CanNotReadFile(error) => {
                format!("unable to read file. got=\"{}\"", error)
            }
            CompilerException::DoubleParameterName(param) => {
                format!("parameter name already in use. got=\"{}\"", param)
            }
            CompilerException::CallingNonFunction(f) => {
                format!("you are attempting to call a non function. got=\"{}\". expected=\"Function(...)\"", f)
            }
            CompilerException::WrongType(got, expected) => {
                format!(
                    "type mismatch, can not assign different type. got=\"{}\". expected\"{}\"",
                    got, expected
                )
            }
            CompilerException::Unknown => "got an error, unknown what went wrong".to_string(),
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
