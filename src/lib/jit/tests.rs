#[cfg(test)]
mod tests {
    use crate::{compiler, lexer, parser};
    use crate::lib::exception::Exception;
    use crate::lib::object::integer::Integer;
    use crate::lib::object::Object;
    use crate::vm::build_vm;


    #[test]
    fn multiply_function() {
        test_jit("var t = fn(x) { return x * 2 }; t(10)", Object::Integer(Integer { value: 20 }))
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

        let mut comp = compiler::build_compiler(None);
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