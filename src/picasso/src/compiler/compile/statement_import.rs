use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::Compiler;
use crate::exception::compiler::{CompilerException, CompilerExceptionCode};
use crate::lexer::build_lexer;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::statement::import::Import;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;
use crate::parser::{build_parser, expression};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn compile_import_statement(
    compiler: &mut Compiler,
    import: Import,
) -> Result<Types, CompilerException> {
    let import_as = import.identifier.clone();
    let file_path = import.file.clone();

    // Find the file, based on the current location of the compiler
    let compiler_location = Path::new(compiler.location.as_str());
    let base_path: &Path = if compiler_location.display().to_string().is_empty() {
        Path::new(compiler.base_location.as_str())
    } else {
        Path::new(compiler_location.parent().unwrap())
    };

    let path = Path::new(base_path).join(Path::new(file_path.as_str()));
    let extension: Option<&str> = path.extension().and_then(OsStr::to_str);

    // Check if path ends with ".loop" or ".lp"
    if let Some(extension) = extension {
        if extension == "loop" || extension == "lp" {
            let path_as_string = path.to_str().unwrap().to_string();

            // Check if file exists
            if !path.exists() {
                return Err(CompilerException::new(
                    0,
                    0,
                    CompilerExceptionCode::CanNotReadFile(path_as_string),
                ));
            }

            let contents = fs::read_to_string(path.clone());

            let contents = {
                if let Ok(contents) = contents {
                    contents
                } else {
                    return Err(CompilerException::new(
                        0,
                        0,
                        CompilerExceptionCode::CanNotReadFile(path_as_string),
                    ));
                }
            };

            // Parse the file
            let lexer = build_lexer(contents.as_str());
            let mut parser = build_parser(lexer, path.to_str().unwrap());

            let program = parser.parse()?;

            compiler.enter_location(path_as_string);
            compiler.compiled_from = contents.clone();

            let result = compiler.compile(program);

            if let Err(result) = result {
                return Err(result);
            }

            let variables = compiler.exit_location();

            let assign = VariableDeclaration {
                ident: Identifier::new(import_as, 0, 0),
                // Value is irrelevant, but required
                value: Box::new(expression::Expression::Integer(Integer { value: 0 })),
                data_type: Types::Module(variables),
                location: (-1, 0),
            };

            return compile_statement_variable_declaration(compiler, assign);
        }
    }

    compiler.imports.push(import_as);

    compiler.add_to_current_function(format!(
        ".LOADLIB {{.CONSTANT CHAR[] \"{}\";}} \"{}\";",
        path.to_str().unwrap(),
        import.identifier
    ));

    Ok(Types::Void)
}
