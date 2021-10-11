#[cfg(test)]
mod tests {
    use crate::object::integer;
    use crate::object::null;
    use crate::object::Object;
    use crate::object::Object::Integer;
    use crate::object::Object::Null;
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};

    #[test]
    fn recursive_functions() {}

    #[test]
    fn expressions() {
        test_vm("100", Integer(integer::Integer { value: 100 }));
        test_vm("100 + 100", Integer(integer::Integer { value: 200 }));
        test_vm("100 / 100", Integer(integer::Integer { value: 1 }));
        test_vm("100 * 2", Integer(integer::Integer { value: 200 }));
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

    // TODO: Add block scoping tests (for variables)

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

    fn test_vm(input: &str, expected: Object) {
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

        let mut vm = build_vm(comp.get_bytecode(), None);
        let err = vm.run();

        if err.is_some() {
            panic!("{}", err.unwrap());
        }

        let last_popped = vm.last_popped;

        if last_popped.is_none() {
            panic!("{}", "VM did not return. got=NONE");
        }

        assert_eq!(*last_popped.unwrap(), expected);
    }
}
