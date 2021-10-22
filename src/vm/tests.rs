#[cfg(test)]
mod tests {
    use crate::lib::exception::Exception;
    use crate::lib::object::null;
    use crate::lib::object::Object;
    use crate::lib::object::Object::Float;
    use crate::lib::object::Object::Integer;
    use crate::lib::object::Object::Null;
    use crate::lib::object::{float, integer};
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};
    use std::borrow::Borrow;
    use std::ops::Deref;

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
        test_vm("-100 / 100", Integer(integer::Integer { value: -1 }));
        //test_vm("10 / 100", Float(float::Float { value: 0.1 }));
        //test_vm("10 / 25", Float(float::Float { value: 0.4 }));
    }

    /*#[test]
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
    }*/

    #[test]
    fn modulo() {
        test_vm("10 % 10", Integer(integer::Integer { value: 0 }));
        test_vm("10 % 4", Integer(integer::Integer { value: 2 }));
        test_vm("10 % 10000", Integer(integer::Integer { value: 10 }));
    }

    fn test_vm(input: &str, expected: Object) {
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

        let mut vm = build_vm(comp.get_bytecode(), None);
        let err = vm.run(false);

        if err.is_err() {
            panic!("{}", err.err().unwrap());
        }

        let got = err.ok().unwrap().clone();

        assert_eq!(*got, expected);
    }
}
