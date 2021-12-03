#[cfg(test)]
mod tests {
    use crate::compiler::instructions::pretty_print_instructions;
    use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
    use crate::lib::exception::Exception;
    use crate::lib::object::Object;
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
[4] OpFunction 1 0
[10] OpReturn",
        );

        expected.push(
            "\
[0] OpGetLocal 0
[2] OpFunction 2 0
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
    fn extension_methods() {
        let input = "123.to_string(); \"123\".to_int()";

        let expected = "[0] OpConstant 1
[5] OpCallExtension 0 0
[8] OpPop
[9] OpConstant 2
[14] OpCallExtension 1 1
[17] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn extension_method_chained() {
        let input = "123.to_string().to_int().to_string()";

        let expected = "[0] OpConstant 1
[5] OpCallExtension 0 0
[8] OpCallExtension 1 1
[11] OpCallExtension 0 0
[14] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn array() {
        let input = "[0, 1]";

        let expected = "[0] OpConstant 1
[5] OpConstant 2
[10] OpArray 2
[13] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn array_assign() {
        let input = "[0, 1][0] = 1";

        let expected = "[0] OpConstant 1
[5] OpConstant 2
[10] OpArray 2
[13] OpConstant 3
[18] OpConstant 4
[23] OpAssignIndex
[24] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn array_assign_3d() {
        let input = "[[[0, 1]]][0][0][0] = 1";

        let expected = "[0] OpConstant 1
[5] OpConstant 2
[10] OpArray 2
[13] OpArray 1
[16] OpArray 1
[19] OpConstant 3
[24] OpIndex
[25] OpConstant 4
[30] OpIndex
[31] OpConstant 5
[36] OpConstant 6
[41] OpAssignIndex
[42] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn loop_while() {
        let input = "for(true) {}";

        let expected = "[0] OpConstant 1
[5] OpJumpIfFalse 20
[10] OpConstant 0
[15] OpJump 0
[20] OpConstant 0
[25] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn loop_iterator() {
        let input = "for(var i = 0 to 100) {}";

        let expected = "[0] OpConstant 1
[5] OpSetVar 0
[10] OpConstant 3
[15] OpGetVar 0
[20] OpAdd
[21] OpSetVar 0
[26] OpGetVar 0
[31] OpConstant 2
[36] OpGreaterThan
[37] OpJumpIfFalse 10
[42] OpConstant 0
[47] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn loop_array_iterator() {
        let input = "for(var i in []) {}";

        let expected = "[0] OpArray 0
[3] OpSetVar 0
[8] OpConstant 1
[13] OpSetVar 2
[18] OpGetBuiltin 0
[20] OpGetVar 0
[25] OpCall 1
[27] OpGetVar 2
[32] OpGreaterThan
[33] OpJumpIfFalse 80
[38] OpGetVar 0
[43] OpGetVar 2
[48] OpIndex
[49] OpSetVar 1
[54] OpConstant 0
[59] OpConstant 2
[64] OpGetVar 2
[69] OpAdd
[70] OpSetVar 2
[75] OpJump 18
[80] OpConstant 0
[85] OpPop";

        compiler_test(input, expected);
    }

    #[test]
    fn hashmaps() {
        let input = "{20: 30, \"hello test\": { 100: 20 }, false: true }";
        let expected = "[0] OpConstant 1
[5] OpConstant 2
[10] OpConstant 3
[15] OpConstant 4
[20] OpConstant 5
[25] OpHashmap 1
[28] OpConstant 6
[33] OpConstant 7
[38] OpHashmap 3
[41] OpPop";

        // TODO: Look at a way to test this, as hashmaps don't have an order we can't test in what order these opcodes are executed
        //compiler_test(input, expected);
    }

    #[test]
    fn scoping_rules_1() {
        compiler_test_error("var test = 100; if(true) { test }", None);
    }

    #[test]
    fn scoping_rules_2() {
        compiler_test_error(
            "var test = 100; if(true) { var test2 = test }; test2",
            Some(CompilerException::UnknownSymbol(UnknownSymbol {
                name: "test2".to_string(),
                scope_depth: 0,
            })),
        );
    }

    #[test]
    fn scoping_rules_3() {
        compiler_test_error(
            "var test = 100; if(true) { var test2 = 300; if(true) { var test3 = test2 } test3; };",
            Some(CompilerException::UnknownSymbol(UnknownSymbol {
                name: "test3".to_string(),
                scope_depth: 0,
            })),
        );
    }

    #[test]
    fn scoping_rules_4() {
        compiler_test_error("var test = 100; if(true) { var test2 = 300; if(true) { var test3 = test2 } test2; }; test3", Some(CompilerException::UnknownSymbol(UnknownSymbol {
            name: "test3".to_string(),
            scope_depth: 0
        })),);
    }

    #[test]
    fn scoping_rules_functions_1() {
        compiler_test_error(
            "\
        var test = 300;
        var func = fn() {\
        var hello = test + 3;\
        }\
        hello;
        ",
            Some(CompilerException::UnknownSymbol(UnknownSymbol {
                name: "hello".to_string(),
                scope_depth: 0,
            })),
        );
    }

    #[test]
    fn scoping_rules_functions_2() {
        compiler_test_error(
            "\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            var func2 = fn() {
                var hello2 = hello + 200;
            };
            hello2;
        }\
        ",
            Some(CompilerException::UnknownSymbol(UnknownSymbol {
                name: "hello2".to_string(),
                scope_depth: 1,
            })),
        );
    }

    #[test]
    fn scoping_rules_functions_2_1() {
        compiler_test_error(
            "\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            var func2 = fn() {
                var hello2 = hello + 200;
                hello2;
            };
        }\
        ",
            None,
        );
    }

    #[test]
    fn scoping_rules_functions_3() {
        compiler_test_error(
            "\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            if(true) {
                var hello2 = hello + 200;
            };
            hello2;
        }\
        ",
            Some(CompilerException::UnknownSymbol(UnknownSymbol {
                name: "hello2".to_string(),
                scope_depth: 1,
            })),
        );
    }

    #[test]
    fn scoping_rules_functions_3_1() {
        compiler_test_error(
            "\
        var test = 300;
        var func = fn() {\
            var hello = test + 3;\
            if(true) {
                var hello2 = hello + 200;
                hello2;
            };
        }\
        ",
            None,
        );
    }

    //#[test]
    //fn divide_by_zero_integer() {
    //    compiler_test_error("100 / 0", Some(CompilerException::DivideByZero))
    //}

    //#[test]
    //fn divide_by_zero_float() {
    //    compiler_test_error("302 / 0.0", Some(CompilerException::DivideByZero))
    //}

    #[test]
    fn divide_by_integer() {
        compiler_test_error("100 / 2", None)
    }

    //#[test]
    //fn divide_by_float() { compiler_test_error("302 / 1.14", None) }

    fn compiler_test_error(input: &str, expected: Option<CompilerException>) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                if let Exception::Parser(err) = err {
                    println!("ParserException: {}", err);
                }
            }

            panic!("Parser exceptions occurred!")
        }

        let mut comp = compiler::build_compiler(None, false);
        let err = comp.compile(program);

        if expected.is_some() && err.is_ok() {
            panic!("expected error to be \"{:?}\". got=NULL", expected.unwrap())
        }

        if expected.is_some() && err.is_err() {
            assert_eq!(expected.unwrap(), err.err().unwrap())
        }
    }

    fn compiler_test_constants(input: &str, expected: Vec<&str>) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                if let Exception::Parser(err) = err {
                    println!("ParserException: {}", err);
                }
            }

            panic!("Parser exceptions occurred!")
        }

        let mut comp = compiler::build_compiler(None, false);
        comp.compile(program);

        let mut i = 0;
        for constant in comp.constants {
            if let Object::CompiledFunction(func) = &*constant.as_ref().borrow() {
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
                if let Exception::Parser(err) = err {
                    println!("ParserException: {}", err);
                }
            }

            panic!("Parser exceptions occurred!")
        }

        let mut comp = compiler::build_compiler(None, false);
        comp.compile(program);

        let scope = comp.scope();
        let sc = scope.borrow();
        let ins = sc.instructions.clone();

        assert_eq!(expected.to_string(), pretty_print_instructions(ins));
    }
}
