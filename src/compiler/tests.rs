#[cfg(test)]
mod tests {
    use crate::compiler::instructions::{pretty_print_instructions};
    use crate::{compiler, lexer, parser};

    #[test]
    fn emit_instruction() {
        let input = "1; 2";
        let expected = "[0] OpConstant 0
[3] OpPop
[4] OpConstant 1
[7] OpPop";

        compiler_test(input, expected)
    }

    fn compiler_test(input: &str, expected: &str) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        let mut comp = compiler::build_compiler();
        comp.compile(program);

        assert_eq!(
            expected.to_string(),
            pretty_print_instructions(comp.instructions)
        );
    }
}
