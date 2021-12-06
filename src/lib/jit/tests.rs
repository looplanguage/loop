#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use inkwell::context::Context;
    use inkwell::OptimizationLevel;
    use inkwell::passes::PassManager;
    use crate::lib::exception::Exception;
    use crate::lib::object::integer::Integer;
    use crate::lib::object::Object;
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};
    use crate::lib::jit::CodeGen;

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

    fn test_jit(input: &str, expected: Object) {
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

        let mut comp = compiler::build_compiler(None, true);
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
            .ok_or_else(|| "cannot start jit!".to_string()).unwrap();

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
            compiled_functions: HashMap::new(),
            parameters: vec![],
            jit_variables: HashMap::new(),
            last_popped: None
        };

        let err = vm.run(true, &mut codegen);

        if err.is_err() {
            panic!("{}", err.err().unwrap());
        }

        let cloned_err = err.ok().unwrap().clone();

        let got = &*cloned_err.as_ref().borrow();

        assert_eq!(*got, expected);
    }
}
