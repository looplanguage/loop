use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::lexer::build_lexer;
use crate::parser::expression::function::Call;
use crate::parser::expression::identifier::Identifier;
use crate::parser::statement::import::Import;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;
use crate::parser::{build_parser, expression};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn compile_import_statement(compiler: &mut Compiler, import: Import) -> CompilerResult {
    let import_as = import.identifier.clone();
    let file_path = import.file.clone();

    // Find the file, based on the current location of the compiler
    let path = Path::new(compiler.location.as_str()).join(Path::new(file_path.as_str()));

    let extension: Option<&str> = path.extension().and_then(OsStr::to_str);

    // Check if path ends with ".loop" or ".lp"
    if let Some(extension) = extension {
        if extension == "loop" || extension == "lp" {
            let path_as_string = path.to_str().unwrap().to_string();

            // Check if file exists
            if !path.exists() {
                return CompilerResult::Exception(CompilerException::CanNotReadFile(
                    path_as_string,
                ));
            }

            let contents = fs::read_to_string(path.clone());

            let contents = {
                if let Ok(contents) = contents {
                    contents
                } else {
                    return CompilerResult::Exception(CompilerException::CanNotReadFile(
                        path_as_string,
                    ));
                }
            };

            // Parse the file
            let lexer = build_lexer(contents.as_str());
            let mut parser = build_parser(lexer);

            let program = parser.parse();

            compiler.enter_location(path_as_string.clone());

            let result = compiler.compile(program);
            if let Err(result) = result {
                return CompilerResult::Exception(result);
            }

            compiler.exit_location();

            let export = compiler.resolve_with_location(&"__export".to_string(), &path_as_string);

            // Now define it here
            if let Some(export) = export {
                let assign = VariableDeclaration {
                    ident: Identifier { value: import_as },
                    value: Box::new(expression::Expression::Call(Call {
                        identifier: Box::new(expression::Expression::Identifier(Identifier {
                            value: export.name.clone(),
                        })),
                        parameters: vec![expression::Expression::Identifier(Identifier {
                            value: export.name,
                        })],
                    })),
                    data_type: export._type,
                };

                return compile_statement_variable_declaration(compiler, assign);
            }

            return CompilerResult::Success(Types::Void);
        }
    }

    compiler.imports.push(import_as);

    compiler.add_to_current_function(format!(
        ".LOADLIB {{.CONSTANT CHAR[] \"{}\";}} \"{}\";",
        path.to_str().unwrap(),
        import.identifier
    ));

    CompilerResult::Success(Types::Void)
}
