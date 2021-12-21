#[cfg(test)]
mod tests {
    use crate::lib::config::CONFIG;
    use crate::lib::exception::Exception;
    use crate::lib::jit::CodeGen;
    use crate::lib::object::integer::Integer;
    use crate::lib::object::Object;
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};
    use inkwell::context::Context;
    use inkwell::passes::PassManager;
    use inkwell::OptimizationLevel;
    use std::collections::HashMap;
    use std::env;

    #[test]
    fn function_single_parameter() {
        test_jit(
            "var t = fn(x) { return x * 2 }; t(10)",
            Object::Integer(Integer { value: 20 }),
        )
    }

    #[test]
    fn conditionals_less_than() {
        test_jit(
            "var t = fn(x) { if(x < 10) { return 500 } else { return 200 } }; t(9)",
            Object::Integer(Integer { value: 500 }),
        );
        test_jit(
            "var t = fn(x) { if(x < 10) { return 500 } else { return 200 } }; t(10)",
            Object::Integer(Integer { value: 200 }),
        );
    }

    #[test]
    fn conditionals_equals() {
        test_jit(
            "var t = fn(x) { if(x == 10) { return 500 } else { return 200 } }; t(9)",
            Object::Integer(Integer { value: 200 }),
        );
        test_jit(
            "var t = fn(x) { if(x == 10) { return 500 } else { return 200 } }; t(10)",
            Object::Integer(Integer { value: 500 }),
        );
    }

    #[test]
    fn conditionals_nested() {
        test_jit(
            "var t = fn() { if(false) { return 10 } else if(true) { if(false) { return 20 } else { return 30 } } else { return 40 } return 50 }; t()", Object::Integer(Integer { value: 30 })
        )
    }

    #[test]
    fn recursive_fibonacci() {
        test_jit(
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
            Object::Integer(Integer { value: 89 }),
        );
    }

    #[test]
    fn many_functions() {
        test_jit(
            "
            var fib = fn(x) {
                if(x < 2) {
                    return 1
                } else {
                    return fib(x - 1) + fib(x - 2)
                }
            }

            var a = fn(x, y) { return x * y }
            var b = fn(a, b, c) { return a + b + c }
            
            fib(10) + b(a(2, 2), 4, 2) + a(10, 2)
        ",
            Object::Integer(Integer { value: 119 }),
        )
    }

    #[test]
    fn mutable_variables() {
        test_jit(
            "var x = 300; x = x + 10; x",
            Object::Integer(Integer { value: 310 }),
        );
        test_jit(
            "var x = 300; var add = fn(y) { x = x + y; return 1 }; add(10); add(10); x",
            Object::Integer(Integer { value: 320 }),
        );
    }

    #[test]
    fn logical_operators_and() {
        test_jit("true and false", Object::Integer(Integer { value: 0 }));
        test_jit(
            "true and true and true and true",
            Object::Integer(Integer { value: 1 }),
        );
        test_jit(
            "(true and false) and true and true and (true and true)",
            Object::Integer(Integer { value: 0 }),
        );
        test_jit(
            "(true and true) and true and true and (true and true)",
            Object::Integer(Integer { value: 1 }),
        );
    }

    #[test]
    fn logical_operators_or() {
        test_jit("true or false", Object::Integer(Integer { value: 1 }));
        test_jit("true or true", Object::Integer(Integer { value: 1 }));
        test_jit(
            "(true or false) or (false or true or false or false)",
            Object::Integer(Integer { value: 1 }),
        );
    }

    #[test]
    fn logical_operators_combined() {
        test_jit(
            "((true or false) and (true and false)) or true",
            Object::Integer(Integer { value: 1 }),
        );
        test_jit(
            "((true or false) and (true and false)) and true",
            Object::Integer(Integer { value: 0 }),
        );
    }

    fn test_jit(input: &str, expected: Object) {
        if let Ok(e) = env::var("TEST_JIT") {
            if e == "0" {
                assert!(true);
                return;
            }
        } else {
            assert!(true);
            return;
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

        let mut vm = build_vm(comp.get_bytecode(), None, "MAIN".to_string());

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

        let mut codegen = CodeGen {
            context: &context,
            module: &module,
            builder: context.create_builder(),
            execution_engine,
            fpm: &fpm,
            last_popped: None,
            jumps: Vec::new()
        };

        let err = vm.run(Some(codegen));

        if err.is_err() {
            panic!("{}", err.err().unwrap());
        }

        let cloned_err = err.ok().unwrap().clone();

        let got = &*cloned_err.as_ref().borrow();

        assert_eq!(*got, expected);
    }
}
