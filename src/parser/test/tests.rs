#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::parser;
    use crate::parser::expression::boolean::Boolean;
    use crate::parser::expression::conditional::Conditional;
    use crate::parser::expression::function::Function;
    use crate::parser::expression::identifier::Identifier;
    use crate::parser::expression::integer::Integer;
    use crate::parser::expression::suffix::Suffix;
    use crate::parser::program::Node;
    use crate::parser::statement::assign::VariableAssign;
    use crate::parser::statement::block::Block;
    use crate::parser::statement::expression::Expression;
    use crate::parser::statement::return_statement::ReturnStatement;
    use crate::parser::statement::variable::VariableDeclaration;
    use crate::parser::statement::Statement;
    use crate::parser::test::test_helper::test_helper;

    #[test]
    fn functions_return() {
        let input = "fn() {\
        return 20;\
        }";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::from(parser::expression::Expression::Function(Function {
                parameters: vec![],
                body: Block {
                    statements: vec![Statement::Return(ReturnStatement {
                        expression: Box::new(parser::expression::Expression::Integer(Integer {
                            value: 20,
                        })),
                    })],
                },
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn variable_assignment() {
        let input = "\
        var test = 0;\
        var yeet = 500;\
        test = 1000;\
        yeet = test * 2;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "test".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 0,
            })),
        }));

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "yeet".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 500,
            })),
        }));

        expected.push(Statement::VariableAssign(VariableAssign {
            ident: Identifier {
                value: "test".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 1000,
            })),
        }));

        expected.push(Statement::VariableAssign(VariableAssign {
            ident: Identifier {
                value: "yeet".to_string(),
            },
            value: Box::new(parser::expression::Expression::Suffix(Box::from(Suffix {
                left: parser::expression::Expression::Identifier(Identifier {
                    value: "test".to_string(),
                }),
                operator: "*".to_string(),
                right: parser::expression::Expression::Integer(Integer { value: 2 }),
            }))),
        }));

        test_parser(input, expected);
    }

    #[test]
    fn conditionals() {
        let input = "\
        if(true) {} \
        if(false) {} else if(true) {}\
        if(false) {} else if(false) {} else {}\
        if(false) { 1; true; } else if(false) {true; 1 + 1} else { true; }\
        ";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(test_helper::generate_if_expression(
            true,
            Block { statements: vec![] },
            Box::new(None),
        ));

        let else_conditional = test_helper::generate_else_condition(
            true,
            Block { statements: vec![] },
            Box::new(None),
        );
        expected.push(test_helper::generate_if_expression(
            false,
            Block { statements: vec![] },
            else_conditional,
        ));

        let else_conditional = test_helper::generate_else_condition(
            false,
            Block { statements: vec![] },
            test_helper::generate_else_block_box(vec![]),
        );
        expected.push(test_helper::generate_if_expression(
            false,
            Block { statements: vec![] },
            else_conditional,
        ));

        // TODO: This is a hot mess. It is kind of readable but not really...
        let else_conditional = test_helper::generate_else_condition(
            false,
            test_helper::generate_else_block(vec![
                test_helper::generate_boolean_expression_box(true),
                Statement::Expression(Box::new(Expression {
                    expression: Box::new(test_helper::generate_expression_suffix(1, '+', 1)),
                })),
            ]),
            test_helper::generate_else_block_box(vec![Statement::Expression(Box::new(
                Expression {
                    expression: Box::new(test_helper::generate_boolean_expression(true)),
                },
            ))]),
        );

        expected.push(test_helper::generate_if_expression(
            false,
            Block {
                statements: vec![
                    test_helper::generate_integer_expression_box(1),
                    test_helper::generate_boolean_expression_box(true),
                ],
            },
            else_conditional,
        ));

        test_parser(input, expected);
    }

    #[test]
    fn functions() {
        let input = "
    fn() {}\
    fn(a) {}\
    fn(a, b, c, d) {}\
    fn() {\
        1\
    }\
    var functionWithParameters = fn(a, b, c, d) {\
        a + b;\
        var e = c + d;\
    }\
    ";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        let parameters: Vec<Identifier> = vec![];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #2
        let parameters: Vec<Identifier> = vec![test_helper::generate_identifier_v3("a")];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #3
        let parameters: Vec<Identifier> = vec![
            test_helper::generate_identifier_v3("a"),
            test_helper::generate_identifier_v3("b"),
            test_helper::generate_identifier_v3("c"),
            test_helper::generate_identifier_v3("d"),
        ];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #4
        let parameters: Vec<Identifier> = vec![];
        let statements: Vec<Statement> = vec![test_helper::generate_expression_statement_v3(
            test_helper::generate_integer_expression(1),
        )];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #5
        let parameters: Vec<Identifier> = vec![
            test_helper::generate_identifier_v3("a"),
            test_helper::generate_identifier_v3("b"),
            test_helper::generate_identifier_v3("c"),
            test_helper::generate_identifier_v3("d"),
        ];
        let left = test_helper::generate_identifier_expression_v3("a");
        let right = test_helper::generate_identifier_expression_v3("b");
        let left2 = test_helper::generate_identifier_expression_v3("c");
        let right2 = test_helper::generate_identifier_expression_v3("d");
        let statements = vec![
            test_helper::generate_expression_statement_v3(
                test_helper::generate_suffix_expression_v3(left, "+", right),
            ),
            test_helper::generate_variable_declaration_v3(
                "e",
                test_helper::generate_suffix_expression_v3(left2, "+", right2),
            ),
        ];
        let function = test_helper::generate_function_v3(parameters, statements);
        let result =
            test_helper::generate_variable_declaration_v3("functionWithParameters", function);
        expected.push(result);

        test_parser(input, expected);
    }

    #[test]
    fn booleans() {
        let input = "true; false;";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        expected.push(test_helper::generate_boolean_expression_box(true));

        // Test #2
        expected.push(test_helper::generate_boolean_expression_box(false));

        test_parser(input, expected);
    }

    #[test]
    fn booleans_inverted() {
        let input = "!true; !false;";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        expected.push(test_helper::generate_boolean_expression_box(false));

        // Test #2
        expected.push(test_helper::generate_boolean_expression_box(true));

        test_parser(input, expected);
    }

    #[test]
    fn comparison() {
        let input = "true == true; 1 == true; 3 > 4; 3 < 4;";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        let left = test_helper::generate_boolean_expression(true);
        let op = "==";
        let right = test_helper::generate_boolean_expression(true);
        expected.push(test_helper::generate_comparison_v3(left, op, right));

        // Test #2
        let left = test_helper::generate_integer_expression(1);
        let op = "==";
        let right = test_helper::generate_boolean_expression(true);
        expected.push(test_helper::generate_comparison_v3(left, op, right));

        // Test #3
        let left = test_helper::generate_integer_expression(3);
        let op = ">";
        let right = test_helper::generate_integer_expression(4);
        expected.push(test_helper::generate_comparison_v3(left, op, right));

        // Test #4
        let left = test_helper::generate_integer_expression(3);
        let op = "<";
        let right = test_helper::generate_integer_expression(4);
        expected.push(test_helper::generate_comparison_v3(left, op, right));

        test_parser(input, expected);
    }

    #[test]
    fn variable_declaration() {
        let input = "var test = 1;
        var test2 = 40;
        var test3 = 10 * 2;
        ";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        expected.push(test_helper::generate_variable_declaration_v3(
            "test",
            test_helper::generate_integer_expression(1),
        ));

        // Test #2
        expected.push(test_helper::generate_variable_declaration_v3(
            "test2",
            test_helper::generate_integer_expression(40),
        ));

        // Test #3
        let left = test_helper::generate_integer_expression(10);
        let right = test_helper::generate_integer_expression(2);
        expected.push(test_helper::generate_variable_declaration_v3(
            "test3",
            test_helper::generate_suffix_expression_v3(left, "*", right),
        ));

        test_parser(input, expected);
    }

    fn test_parser(input: &str, expected: Vec<Statement>) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                println!("ParserException: {}", err);
            }

            panic!("Parser exceptions occurred!")
        }

        let mut i = 0;
        for statement in program.statements {
            assert_eq!(statement, expected[i]);

            i += 1;
        }
    }
}
