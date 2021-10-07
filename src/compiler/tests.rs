#[cfg(test)]
mod tests {
    use crate::compiler::instructions::pretty_print_instructions;
    use crate::{compiler, lexer, parser};

    #[test]
    fn emit_instruction() {
        let input = "1; 2";
        let expected = "[0] OpConstant 0
[5] OpPop
[6] OpConstant 1
[11] OpPop";

        compiler_test(input, expected)
    }

    fn compiler_test(input: &str, expected: &str) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                println!("ParserException: {}", err);
            }

            panic!("Parser exceptions occurred!")
        }

        let mut comp = compiler::build_compiler(None);
        comp.compile(program);

        assert_eq!(
            expected.to_string(),
            pretty_print_instructions(comp.instructions)
        );
    }
}
