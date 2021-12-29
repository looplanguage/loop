#[cfg(test)]
mod tests {
    use crate::lib::exception::Exception;
    use crate::lib::jit::CodeGen;
    use crate::lib::object::Object;
    use crate::lib::object::Object::Null;
    use crate::lib::object::Object::String;
    use crate::lib::object::Object::{Array, Integer};
    use crate::lib::object::Object::{Boolean, Float};
    use crate::lib::object::{array, null};
    use crate::lib::object::{boolean, float, integer, string};
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};
    use inkwell::context::Context;
    use inkwell::passes::PassManager;
    use inkwell::OptimizationLevel;
    use std::cell::RefCell;
    use std::env;
    use std::rc::Rc;

    #[test]
    fn recursive_functions() {}

    #[test]
    fn strings() {
        test_vm(
            "\"hello\"",
            String(string::LoopString {
                value: "hello".parse().unwrap(),
            }),
        );
        test_vm(
            "\"Hello\"",
            String(string::LoopString {
                value: "Hello".parse().unwrap(),
            }),
        );
        test_vm(
            "\"123\"",
            String(string::LoopString {
                value: "123".parse().unwrap(),
            }),
        );
        test_vm(
            "\"I123\"",
            String(string::LoopString {
                value: "I123".parse().unwrap(),
            }),
        );
    }

    #[test]
    fn escape_sequences() {
        test_vm(
            "\"x\\ny\"",
            String(string::LoopString {
                value: "x\ny".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\ry\"",
            String(string::LoopString {
                value: "x\ry".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\ty\"",
            String(string::LoopString {
                value: "x\ty".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\\\y\"",
            String(string::LoopString {
                value: "x\\\\y".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\\"y\"",
            String(string::LoopString {
                value: "x\"y".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\\'y\"",
            String(string::LoopString {
                value: "x\'y".parse().unwrap(),
            }),
        );
        test_vm(
            "\"x\\y\"",
            String(string::LoopString {
                value: "x\\y".parse().unwrap(),
            }),
        );
    }

    #[test]
    fn expressions() {
        test_vm("100", Integer(integer::Integer { value: 100 }));
        test_vm("100 + 100", Integer(integer::Integer { value: 200 }));
        test_vm("100 / 100", Integer(integer::Integer { value: 1 }));
        test_vm("100 * 2", Integer(integer::Integer { value: 200 }));
        test_vm("100 ^ 2", Integer(integer::Integer { value: 10000 }));
        test_vm("1.5 ^ 2", Float(float::Float { value: 2.25 }));
        test_vm(
            "2 ^ 1.5",
            Float(float::Float {
                value: 2.8284271247461903,
            }),
        );
    }

    #[test]
    fn expression_precedence() {
        test_vm("100 + 1 * 2", Integer(integer::Integer { value: 102 }));
        test_vm("(100 + 1) * 2", Integer(integer::Integer { value: 202 }));
        test_vm(
            "((100 + 1) * 2) + 500 + (300 * 2 / 15) * 10",
            Integer(integer::Integer { value: 1102 }),
        );
    }

    #[test]
    fn variable_declaration() {
        test_vm(
            "var test = 100; test * 2;",
            Integer(integer::Integer { value: 200 }),
        );
        test_vm(
            "var test = 1000; test = 500; test / 2",
            Integer(integer::Integer { value: 250 }),
        );
    }

    #[test]
    fn block_scope_1() {
        test_vm(
            "var test = 100; if(true) { test = 1000 }; test",
            Integer(integer::Integer { value: 1000 }),
        );
    }

    // TODO: Add early return tests

    // TODO: Add expression checks for conditionals

    #[test]
    fn conditional() {
        test_vm("if(true) { 10 }", Integer(integer::Integer { value: 10 }));

        test_vm(
            "if(true) { 40 + 40 }",
            Integer(integer::Integer { value: 80 }),
        );

        test_vm(
            "if(true) { 10 * 2 + 1 }",
            Integer(integer::Integer { value: 21 }),
        );

        test_vm(
            "if(false) { 10 * 2 + 1 } else { 20 }",
            Integer(integer::Integer { value: 20 }),
        );

        test_vm(
            "if(false) { 10 * 2 + 1 } else if(false) { 20 } else { 100 }",
            Integer(integer::Integer { value: 100 }),
        );

        test_vm(
            "if(false) { 10 * 2 + 1 } else if(false) { 20 } else if(false) { 100 } else { 300 }",
            Integer(integer::Integer { value: 300 }),
        );

        test_vm(
            "if(false) { 10 * 2 + 1 } else if(true) { 20 } else if(false) { 100 } else { 300 }",
            Integer(integer::Integer { value: 20 }),
        );

        test_vm(
            "if(true) { 10 * 2 + 1 } else if(false) { 20 } else if(true) { 100 } else { 300 }",
            Integer(integer::Integer { value: 21 }),
        );

        test_vm(
            "if(false) { 10 * 2 + 1 } else { if(false) { 100 } else { 400 } }",
            Integer(integer::Integer { value: 400 }),
        );
    }

    #[test]
    fn conditional_null() {
        test_vm("if(false) { 10 }", Null(null::Null {}));
        test_vm("if(false) { 10 } else {}", Null(null::Null {}));
        test_vm(
            "if(false) { 10 } else { if(false) { 10 } }",
            Null(null::Null {}),
        );

        test_vm(
            "if(false) { 10 } else if(false) { if(false) { 10 } } else if(false) {320 + 400} else if(true) {} else { 6000 }",
            Null(null::Null {}),
        );
    }

    #[test]
    fn recursive_fibonacci() {
        test_vm(
            "
            var fib = fn(x) {
                if(x < 2) {
                    return 1
                } else {
                    return fib(x - 1) + fib(x - 2)
                }
            }
            fib(10)
        ",
            Integer(integer::Integer { value: 89 }),
        );
    }

    #[test]
    fn closures_1() {
        test_vm(
            "var newClosure = fn(a) { return fn(b) { return a + b } }; newClosure(20)(10)",
            Integer(integer::Integer { value: 30 }),
        )
    }

    #[test]
    fn closures_2() {
        test_vm(
            "var newClosure = fn(a) { return fn(b) { return fn(c) { return a + b + c } } }; newClosure(30)(20)(10)",
            Integer(integer::Integer { value: 60 }),
        )
    }

    #[test]
    fn closures_3() {
        test_vm(
            "var newClosure = fn(a) { return fn(b) { return fn(c) { return fn(d) { return a + b + c + d } } } }; newClosure(30)(20)(10)(5)",
            Integer(integer::Integer { value: 65 }),
        )
    }

    #[test]
    fn closures_variable_scopes_1() {
        test_vm(
            "var newClosure = fn(a) { var c = 1000; return fn(b) { return a + b + c } }; newClosure(20)(10)",
            Integer(integer::Integer { value: 1030 }),
        )
    }

    #[test]
    fn closures_variable_scopes_2() {
        test_vm(
            "var q = 200; var newClosure = fn(a) { var c = 1000; return fn(b) { return a + b + c + q } }; newClosure(20)(10)",
            Integer(integer::Integer { value: 1230 }),
        )
    }

    #[test]
    fn divisions_integer() {
        test_vm("100 / 2", Integer(integer::Integer { value: 50 }));
        test_vm("100 / 20", Integer(integer::Integer { value: 5 }));
        test_vm("1000 / 250", Integer(integer::Integer { value: 4 }));
        test_vm("100 / -100", Float(float::Float { value: -1.0 }));
        test_vm("-100 / -100", Integer(integer::Integer { value: 1 }));
        test_vm("-100 / 100", Float(float::Float { value: -1.0 }));
        test_vm("10 / 100", Float(float::Float { value: 0.1 }));
        test_vm("10 / 25", Float(float::Float { value: 0.4 }));
    }

    #[test]
    fn division_float() {
        test_vm(
            "10 / 3",
            Float(float::Float {
                value: 3.3333333333333335,
            }),
        );

        test_vm("10 / 2.5", Integer(integer::Integer { value: 4 }));

        test_vm("9 / 2", Float(float::Float { value: 4.5 }));

        test_vm("13 / (7 + 1)", Float(float::Float { value: 1.625 }));
    }
    #[test]
    fn extension_method_array_length() {
        test_vm("[].length()", Integer(integer::Integer { value: 0 }));
        test_vm("[1, 2, 3].length()", Integer(integer::Integer { value: 3 }));
    }

    #[test]
    fn extension_method_array_remove() {
        test_vm(
            "var arr = [1, 2, 3]; arr.remove(2)",
            Integer(integer::Integer { value: 3 }),
        );
        test_vm(
            "var arr = [1, 2, 3]; arr.remove(0); arr[0]",
            Integer(integer::Integer { value: 2 }),
        );
    }

    #[test]
    fn extension_method_array_add() {
        test_vm(
            "var arr = [1, 2, 3]; arr.add(4); arr.length()",
            Integer(integer::Integer { value: 4 }),
        );
        test_vm(
            "var arr = [1, 2, 3]; arr.add(4); arr[3]",
            Integer(integer::Integer { value: 4 }),
        );
    }
    #[test]
    fn extension_method_array_slice() {
        test_vm(
            "var arr = [1, 2, 3]; arr.slice(0, 1); arr.length()",
            Integer(integer::Integer { value: 2 }),
        );
        test_vm(
            "var arr = [1, 2, 3]; arr.slice(1, 2); arr[0]",
            Integer(integer::Integer { value: 2 }),
        );
    }

    #[test]
    fn modulo() {
        test_vm("10 % 10", Integer(integer::Integer { value: 0 }));
        test_vm("10 % 4", Integer(integer::Integer { value: 2 }));
        test_vm("10 % 10000", Integer(integer::Integer { value: 10 }));
    }

    #[test]
    fn extension_methods() {
        test_vm(
            "123.to_string();",
            String(string::LoopString {
                value: "123".to_string(),
            }),
        );

        test_vm(
            "\"123\".to_int();",
            Integer(integer::Integer { value: 123 }),
        );

        test_vm("false.to_int();", Integer(integer::Integer { value: 0 }));

        test_vm("true.to_int();", Integer(integer::Integer { value: 1 }));
    }

    #[test]
    fn builtin_methods() {
        test_vm(
            "format(\"%a %a!\", \"Hello\", \"world\")",
            String(string::LoopString {
                value: "Hello world!".to_string(),
            }),
        )
    }

    #[test]
    fn extension_methods_variables() {
        test_vm(
            "var x = 123; x.to_string();",
            String(string::LoopString {
                value: "123".to_string(),
            }),
        );

        test_vm(
            "var x = \"123\"; x.to_int();",
            Integer(integer::Integer { value: 123 }),
        );
    }

    #[test]
    fn extension_method_chained() {
        test_vm(
            "123.to_string().to_int();",
            Integer(integer::Integer { value: 123 }),
        );
        test_vm(
            "123.to_string().to_int().to_string();",
            String(string::LoopString {
                value: "123".to_string(),
            }),
        );
    }

    #[test]
    fn array_index() {
        test_vm(
            "[1, 2, 3][0]",
            Object::Integer(integer::Integer { value: 1 }),
        );

        test_vm(
            "var x = [1, 2, 3]; x[2]",
            Object::Integer(integer::Integer { value: 3 }),
        );
    }

    #[test]
    fn array_index_deep() {
        test_vm(
            "[[0, 1, 2], [3, 4, 5]][1][0]",
            Object::Integer(integer::Integer { value: 3 }),
        );

        test_vm(
            "var x = [[0, 1, 2], [3, 4, 5]]; x[1][0]",
            Object::Integer(integer::Integer { value: 3 }),
        );
    }

    #[test]
    fn array_assign_index() {
        test_vm(
            "var x = [0, 1, 2]; x[2] = 400; x",
            Array(array::Array {
                values: vec![
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 0,
                    }))),
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 1,
                    }))),
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 400,
                    }))),
                ],
            }),
        );
    }

    #[test]
    fn array_3d_assign_index() {
        test_vm(
            "var x = [[[0, 1, 2]], []]; x[0][0][1] = 200; x",
            Array(array::Array {
                values: vec![
                    Rc::from(RefCell::from(Object::Array(array::Array {
                        values: vec![Rc::from(RefCell::from(Object::Array(array::Array {
                            values: vec![
                                Rc::from(RefCell::from(Object::Integer(integer::Integer {
                                    value: 0,
                                }))),
                                Rc::from(RefCell::from(Object::Integer(integer::Integer {
                                    value: 200,
                                }))),
                                Rc::from(RefCell::from(Object::Integer(integer::Integer {
                                    value: 2,
                                }))),
                            ],
                        })))],
                    }))),
                    Rc::from(RefCell::from(Object::Array(array::Array {
                        values: vec![],
                    }))),
                ],
            }),
        );
    }

    #[test]
    fn arrays() {
        test_vm("[]", Array(array::Array { values: vec![] }));

        test_vm(
            "[1, 2, 3]",
            Array(array::Array {
                values: vec![
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 1,
                    }))),
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 2,
                    }))),
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 3,
                    }))),
                ],
            }),
        );

        test_vm(
            "[1, null, true]",
            Array(array::Array {
                values: vec![
                    Rc::from(RefCell::from(Object::Integer(integer::Integer {
                        value: 1,
                    }))),
                    Rc::from(RefCell::from(Object::Null(null::Null {}))),
                    Rc::from(RefCell::from(Object::Boolean(boolean::Boolean {
                        value: true,
                    }))),
                ],
            }),
        );
    }

    #[test]
    fn extension_method_suffix() {
        test_vm(
            "123.to_string().to_int() == 123",
            Boolean(boolean::Boolean { value: true }),
        );

        test_vm(
            "123.to_string().to_int().to_string() == 123",
            Boolean(boolean::Boolean { value: false }),
        );

        test_vm(
            "123.to_string().to_int().to_string() == \"123\"",
            Boolean(boolean::Boolean { value: true }),
        );

        test_vm(
            "123.to_string().to_int() + 1 == 124",
            Boolean(boolean::Boolean { value: true }),
        );
    }

    #[test]
    fn comments_single_line() {
        test_vm(
            "var x = 10 // This is a comment",
            Integer(integer::Integer { value: 10 }),
        );

        test_vm(
            "// This is a comment \n
             var y = 11",
            Integer(integer::Integer { value: 11 }),
        );
    }

    #[test]
    fn comments_block() {
        test_vm(
            " var y = 10 \
            y = 2\
            /< hello \
            multiline >/",
            Integer(integer::Integer { value: 2 }),
        );

        test_vm(
            "/<hello>/ var x = 11",
            Integer(integer::Integer { value: 11 }),
        );

        test_vm(
            "if(true) { /<hello>/ print(5) }",
            Integer(integer::Integer { value: 5 }),
        );

    }

    #[test]
    fn loop_while() {
        test_vm(
            "var x = 0; for (x < 10) { x = x + 1 }; x",
            Integer(integer::Integer { value: 10 }),
        );
    }

    #[test]
    fn loop_iterator() {
        test_vm(
            "var x = 0; for (var i = 0 to 10) { x = x + 1 }; x",
            Integer(integer::Integer { value: 10 }),
        );
    }

    #[test]
    fn loop_array_iterator() {
        test_vm(
            "var x = 0; var array = [3, 8, 12, 56] for (var i in array) { x = i }; x",
            Integer(integer::Integer { value: 56 }),
        );
    }

    #[test]
    fn hashmap() {
        test_vm(
            "{20: 30, \"hello test\": { 100: 20 }, false: true }[20]",
            Integer(integer::Integer { value: 30 }),
        );
        test_vm(
            "{20: 30, \"hello test\": { 100: 20 }, false: true }[\"hello test\"][100]",
            Integer(integer::Integer { value: 20 }),
        );
        test_vm(
            "{20: 30, \"hello test\": { 100: 20 }, false: true }[false]",
            Boolean(boolean::Boolean { value: true }),
        );
    }

    #[test]
    fn hashmap_assign() {
        test_vm(
            "var x = {0: 300, 2: 500, false: true}; x[0] = 500; x[0]",
            Integer(integer::Integer { value: 500 }),
        );
        test_vm(
            "var x = {0: 300, 2: 500, false: true}; x[2] = false; x[2]",
            Boolean(boolean::Boolean { value: false }),
        );
        test_vm(
            "var x = {0: 300, 2: 500, false: true}; x[false] = false; x[false]",
            Boolean(boolean::Boolean { value: false }),
        );
    }

    #[test]
    fn hashmap_nested_assign() {
        test_vm(
            "var x = { true: { 0: { \"hello\": {30: 500 } } } }; x[true][0][\"hello\"][30] = true; x[true][0][\"hello\"][30]",
            Boolean(boolean::Boolean { value: true }),
        );
    }

    #[test]
    fn logical_operators_and() {
        test_vm("true and false", Boolean(boolean::Boolean { value: false }));
        test_vm(
            "true and true and true and true",
            Boolean(boolean::Boolean { value: true }),
        );
        test_vm(
            "(true and false) and true and true and (true and true)",
            Boolean(boolean::Boolean { value: false }),
        );
        test_vm(
            "(true and true) and true and true and (true and true)",
            Boolean(boolean::Boolean { value: true }),
        );
    }

    #[test]
    fn logical_operators_or() {
        test_vm("true or false", Boolean(boolean::Boolean { value: true }));
        test_vm("true or true", Boolean(boolean::Boolean { value: true }));
        test_vm(
            "(true or false) or (false or true or false or false)",
            Boolean(boolean::Boolean { value: true }),
        );
    }

    #[test]
    fn logical_operators_combined() {
        test_vm(
            "((true or false) and (true and false)) or true",
            Boolean(boolean::Boolean { value: true }),
        );
        test_vm(
            "((true or false) and (true and false)) and true",
            Boolean(boolean::Boolean { value: false }),
        );
    }

    fn test_vm(input: &str, expected: Object) {
        if let Ok(e) = env::var("TEST_JIT") {
            if e == "1" {
                assert!(true);
                return;
            }
        }

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

        let mut comp = compiler::build_compiler(None);
        let err = comp.compile(program);

        if err.is_err() {
            panic!("{:?}", err.err().unwrap());
        }

        let context = Context::create();
        let module = context.create_module("program");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .ok()
            .ok_or_else(|| "cannot start jit!".to_string())
            .unwrap();

        let fpm = PassManager::create(&module);

        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();

        fpm.initialize();

        let codegen = CodeGen {
            context: &context,
            module: &module,
            builder: context.create_builder(),
            execution_engine,
            fpm: &fpm,
            last_popped: None,
        };

        let mut vm = build_vm(comp.get_bytecode(), None, "MAIN".to_string());
        let err = vm.run(Some(codegen));

        if err.is_err() {
            panic!("{}", err.err().unwrap());
        }

        let cloned_err = err.ok().unwrap().clone();

        let got = &*cloned_err.as_ref().borrow();

        assert_eq!(*got, expected);
    }
}
