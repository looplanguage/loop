#[cfg(test)]
mod tests {
    use crate::lib::exception::Exception;
    use crate::lib::object::integer::Integer;
    use crate::lib::object::Object;
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};

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
            "var t = fn(x) { if(x < 10) { 500 } else { 200 } }; t(9)",
            Object::Integer(Integer { value: 500 }),
        );
        test_jit(
            "var t = fn(x) { if(x < 10) { 500 } else { 200 } }; t(10)",
            Object::Integer(Integer { value: 200 }),
        );
    }

    #[test]
    fn conditionals_equals() {
        test_jit(
            "var t = fn(x) { if(x == 10) { 500 } else { 200 } }; t(9)",
            Object::Integer(Integer { value: 200 }),
        );
        test_jit(
            "var t = fn(x) { if(x == 10) { 500 } else { 200 } }; t(10)",
            Object::Integer(Integer { value: 500 }),
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

        let mut vm = build_vm(comp.get_bytecode(), None);
        let err = vm.run(true);

        if err.is_err() {
            panic!("{}", err.err().unwrap());
        }

        let cloned_err = err.ok().unwrap().clone();

        let got = &*cloned_err.as_ref().borrow();

        assert_eq!(*got, expected);
    }
}
