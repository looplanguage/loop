#[cfg(test)]
mod tests {
    use crate::compiler::instructions::pretty_print_instructions;
    use crate::object::Object;
    use crate::{compiler, lexer, parser};
    use std::borrow::Borrow;

    #[test]
    fn closures() {
        let input = "\
        fn(a) {
                return fn(b) {
                    return fn(c) {
                        return a + b + c
                    }
                }
            };\
        ";
        let mut expected: Vec<&str> = Vec::new();
        expected.push(
            "[0] OpGetFree 0
[2] OpGetFree 1
[4] OpAdd
[5] OpGetLocal 0
[7] OpAdd
[8] OpReturn",
        );

        expected.push(
            "\
[0] OpGetFree 0
[2] OpGetLocal 0
[4] OpFunction 1 2
[10] OpReturn",
        );

        expected.push(
            "\
[0] OpGetLocal 0
[2] OpFunction 2 1
[8] OpReturn",
        );

        compiler_test_constants(input, expected);
    }

    #[test]
    fn emit_instruction() {
        let input = "1; 2";
        let expected = "[0] OpConstant 1
[5] OpPop
[6] OpConstant 2
[11] OpPop";

        compiler_test(input, expected)
    }

    #[test]
    fn scoping_rules_1() {
        compiler_test_error("var test = 100; if(true) { test }", None);
    }

    #[test]
    fn scoping_rules_2() {
        compiler_test_error("var test = 100; if(true) { var test2 = test }; test2", Some(String::from("unknown variable. got=\"test2\"")));
    }

    #[test]
    fn scoping_rules_3() {
        compiler_test_error("var test = 100; if(true) { var test2 = 300; if(true) { var test3 = test2 } test3; };", Some(String::from("unknown variable. got=\"test3\"")));
    }

    #[test]
    fn scoping_rules_4() {
        compiler_test_error("var test = 100; if(true) { var test2 = 300; if(true) { var test3 = test2 } test2; }; test3", Some(String::from("unknown variable. got=\"test3\"")));
    }

    #[test]
    fn scoping_rules_functions_1() {
        compiler_test_error("\
        var test = 300;
        var func = fn() {\
        var hello = test + 3;\
        }\
        hello;
        ", Some(String::from("unknown variable. got=\"hello\"")));
    }

    #[test]
    fn scoping_rules_functions_2() {
        compiler_test_error("\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            var func2 = fn() {
                var hello2 = hello + 200;
            };
            hello2;
        }\
        ", Some(String::from("unknown variable. got=\"hello2\"")));
    }

    #[test]
    fn scoping_rules_functions_2() {
        compiler_test_error("\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            if(true) {
                var hello2 = hello + 200;
            };
            hello2;
        }\
        ", Some(String::from("unknown variable. got=\"hello2\"")));
    }

    fn compiler_test_error(input: &str, expected: Option<String>) {
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
        let err = comp.compile(program);

        assert_eq!(expected, err);
    }

    fn compiler_test_constants(input: &str, expected: Vec<&str>) {
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

        let mut i = 0;
        for constant in comp.constants {
            if let Object::CompiledFunction(func) = constant {
                let ins = func.instructions.clone();

                assert_eq!(expected[i - 1].to_string(), pretty_print_instructions(ins));
                i = i + 1 as usize;
            } else {
                i = i + 1;
                continue;
            }
        }
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

        let scope = comp.scope();
        let sc = scope.borrow();
        let ins = sc.instructions.clone();

        assert_eq!(expected.to_string(), pretty_print_instructions(ins));
    }
}
