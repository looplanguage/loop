#[cfg(test)]
mod tests {
    use crate::object::integer;
    use crate::object::Object;
    use crate::object::Object::Integer;
    use crate::vm::build_vm;
    use crate::{compiler, lexer, parser};

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

        assert_eq!(last_popped.unwrap(), expected);
    }
}
