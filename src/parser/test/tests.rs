#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::exception::Exception;
    use crate::parser;
    use crate::parser::expression::array::Array;
    use crate::parser::expression::boolean::Boolean;
    use crate::parser::expression::function::{Call, Function, Parameter};
    use crate::parser::expression::hashmap::{HashableExpression, Hashmap};
    use crate::parser::expression::identifier::Identifier;
    use crate::parser::expression::integer::Integer;
    use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
    use crate::parser::expression::null::Null;
    use crate::parser::expression::string::LoopString;
    use crate::parser::expression::suffix::Suffix;
    use crate::parser::expression::Expression::Index;
    use crate::parser::statement::assign::VariableAssign;
    use crate::parser::statement::block::Block;
    use crate::parser::statement::expression::Expression;
    use crate::parser::statement::return_statement::ReturnStatement;
    use crate::parser::statement::variable::VariableDeclaration;
    use crate::parser::statement::Statement;
    use crate::parser::test::test_helper::test_helper;
    use crate::parser::types::{BaseTypes, Types};
    use std::collections::HashMap;

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
                name: "".to_string(),
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn loop_while() {
        let input = "for(true) { }";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Loop(Loop {
                condition: Box::new(parser::Expression::Boolean(Boolean { value: true })),
                body: Block { statements: vec![] },
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn hashmap() {
        let input = "{\"hello world\": 123, true: 123, 500: false}";

        let mut expected: Vec<Statement> = Vec::new();
        let mut hashmap_values: HashMap<HashableExpression, parser::expression::Expression> =
            HashMap::new();

        hashmap_values.insert(
            HashableExpression::String(LoopString {
                value: "hello world".to_string(),
            }),
            parser::Expression::Integer(Integer { value: 123 }),
        );

        hashmap_values.insert(
            HashableExpression::Boolean(Boolean { value: true }),
            parser::Expression::Integer(Integer { value: 123 }),
        );

        hashmap_values.insert(
            HashableExpression::Integer(Integer { value: 500 }),
            parser::Expression::Boolean(Boolean { value: false }),
        );

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Hashmap(Hashmap {
                values: hashmap_values,
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn loop_iterator() {
        let input = "for(var i = 0 to 100) { }";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::LoopIterator(LoopIterator {
                identifier: Identifier {
                    value: "i".to_string(),
                },
                from: 0,
                till: 100,
                body: Block { statements: vec![] },
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn loop_array_iterator() {
        let input = "for(var value in []) { }";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::LoopArrayIterator(LoopArrayIterator {
                identifier: Identifier {
                    value: "value".to_string(),
                },
                body: Block { statements: vec![] },
                array: Box::new(parser::Expression::Array(Box::new(Array {
                    values: vec![],
                }))),
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn extension_methods_chained() {
        let input = "
    \"123\".to_int().to_string().to_int();
";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(Index(Box::new(parser::expression::index::Index {
                left: Index(Box::new(parser::expression::index::Index {
                    left: Index(Box::new(parser::expression::index::Index {
                        left: parser::expression::Expression::String(LoopString {
                            value: "123".to_string(),
                        }),
                        index: parser::expression::Expression::Call(Call {
                            identifier: Box::new(parser::Expression::Identifier(Identifier {
                                value: "to_int".to_string(),
                            })),
                            parameters: vec![],
                        }),
                    })),
                    index: parser::expression::Expression::Call(Call {
                        identifier: Box::new(parser::Expression::Identifier(Identifier {
                            value: "to_string".to_string(),
                        })),
                        parameters: vec![],
                    }),
                })),
                index: parser::expression::Expression::Call(Call {
                    identifier: Box::new(parser::Expression::Identifier(Identifier {
                        value: "to_int".to_string(),
                    })),
                    parameters: vec![],
                }),
            }))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn extension_methods() {
        let input = "
    \"123\".to_int();
    123.to_string();
";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(Index(Box::new(parser::expression::index::Index {
                left: parser::expression::Expression::String(LoopString {
                    value: "123".to_string(),
                }),
                index: parser::expression::Expression::Call(Call {
                    identifier: Box::new(parser::Expression::Identifier(Identifier {
                        value: "to_int".to_string(),
                    })),
                    parameters: vec![],
                }),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(Index(Box::new(parser::expression::index::Index {
                left: parser::expression::Expression::Integer(Integer { value: 123 }),
                index: parser::expression::Expression::Call(Call {
                    identifier: Box::new(parser::Expression::Identifier(Identifier {
                        value: "to_string".to_string(),
                    })),
                    parameters: vec![],
                }),
            }))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn variable_assignment() {
        let input = "\
        test := 0;\
        yeet := 500;\
        test = 1000;\
        foo = 2 ^ 3;\
        yeet = test * 2;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "test".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 0,
            })),
            data_type: Types::Auto,
        }));

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "yeet".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 500,
            })),
            data_type: Types::Auto,
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
                value: "foo".to_string(),
            },
            value: Box::new(parser::expression::Expression::Suffix(Box::from(Suffix {
                left: parser::expression::Expression::Integer(Integer { value: 2 }),
                operator: "^".to_string(),
                right: parser::expression::Expression::Integer(Integer { value: 3 }),
            }))),
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
        // Same test, only parenthesis are different
        let input = "
        if(false) { 1; true; } else if(false) {true; 1 + 1} else { true; }\
        if false { 1; true; } else if false  {true; 1 + 1} else { true; }\
        ";

        let mut expected: Vec<Statement> = Vec::new();

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
    fn(int a) {}\
    fn(int a, int b, int c, int d) {}\
    fn() {\
        1\
    }\
    functionWithParameters := fn(int a, int b, int c, int d) {\
        a + b;\
        e := c + d;\
    }\
    ";

        let mut expected: Vec<Statement> = Vec::new();

        // Test #1
        let parameters: Vec<Parameter> = vec![];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #2
        let parameters: Vec<Parameter> = vec![test_helper::generate_parameter_v3(
            "a",
            Types::Basic(BaseTypes::Integer),
        )];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #3
        let parameters: Vec<Parameter> = vec![
            test_helper::generate_parameter_v3("a", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("b", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("c", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("d", Types::Basic(BaseTypes::Integer)),
        ];
        let statements: Vec<Statement> = vec![];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #4
        let parameters: Vec<Parameter> = vec![];
        let statements: Vec<Statement> = vec![test_helper::generate_expression_statement_v3(
            test_helper::generate_integer_expression(1),
        )];
        expected.push(Statement::Expression(Box::new(Expression {
            expression: test_helper::generate_function_v3_box(parameters, statements),
        })));

        // Test #5
        let parameters: Vec<Parameter> = vec![
            test_helper::generate_parameter_v3("a", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("b", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("c", Types::Basic(BaseTypes::Integer)),
            test_helper::generate_parameter_v3("d", Types::Basic(BaseTypes::Integer)),
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
    fn strings() {
        let input = "\"hello world!\" \"hello world, from a string!\"";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::new(parser::expression::Expression::String(LoopString {
                value: String::from("hello world!"),
            })),
        })));

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::new(parser::expression::Expression::String(LoopString {
                value: String::from("hello world, from a string!"),
            })),
        })));

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
        let input = "
        test := 1;
        test2 := 40;
        test3 := 10 * 2;
        test4 := 1.1;
        test5 := -1;
        test6 := -1.1;
        test7 := 1.1 + 1.1;
        test8 := 1.1 + 1;
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

        // Test #4
        expected.push(test_helper::generate_variable_declaration_v3(
            "test4",
            test_helper::generate_float_expression(1.1),
        ));

        // Test #5
        expected.push(test_helper::generate_variable_declaration_v3(
            "test5",
            test_helper::generate_integer_expression(-1),
        ));

        // Test #6
        expected.push(test_helper::generate_variable_declaration_v3(
            "test6",
            test_helper::generate_float_expression(-1.1),
        ));

        // Test #7
        let left = test_helper::generate_float_expression(1.1);
        let right = test_helper::generate_float_expression(1.1);
        expected.push(test_helper::generate_variable_declaration_v3(
            "test7",
            test_helper::generate_suffix_expression_v3(left, "+", right),
        ));

        // Test #8
        let left = test_helper::generate_float_expression(1.1);
        let right = test_helper::generate_integer_expression(1);
        expected.push(test_helper::generate_variable_declaration_v3(
            "test8",
            test_helper::generate_suffix_expression_v3(left, "+", right),
        ));

        test_parser(input, expected);
    }

    #[test]
    fn array() {
        let input = "[]";

        let mut expected: Vec<Statement> = vec![];

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::new(parser::expression::Expression::Array(Box::from(Array {
                values: vec![],
            }))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn array_content() {
        let input = "[1, 2, 3]";

        let mut expected: Vec<Statement> = vec![];

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::new(parser::expression::Expression::Array(Box::from(Array {
                values: vec![
                    Expression {
                        expression: Box::from(parser::expression::Expression::Integer(Integer {
                            value: 1,
                        })),
                    },
                    Expression {
                        expression: Box::from(parser::expression::Expression::Integer(Integer {
                            value: 2,
                        })),
                    },
                    Expression {
                        expression: Box::from(parser::expression::Expression::Integer(Integer {
                            value: 3,
                        })),
                    },
                ],
            }))),
        })));

        test_parser(input, expected);
    }

    fn add_array_mixed_content(mut expected: Vec<Statement>) -> Vec<Statement> {
        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::new(parser::expression::Expression::Array(Box::from(Array {
                values: vec![
                    Expression {
                        expression: Box::from(parser::expression::Expression::Integer(Integer {
                            value: 1,
                        })),
                    },
                    Expression {
                        expression: Box::from(parser::expression::Expression::Null(Null {})),
                    },
                    Expression {
                        expression: Box::from(parser::expression::Expression::Boolean(Boolean {
                            value: true,
                        })),
                    },
                    Expression {
                        expression: Box::from(parser::expression::Expression::String(LoopString {
                            value: "hello world".to_string(),
                        })),
                    },
                ],
            }))),
        })));

        expected
    }

    #[test]
    fn array_mixed_content() {
        let input = "[1, null, true, \"hello world\"]";

        let expected: Vec<Statement> = vec![];

        let expected = add_array_mixed_content(expected);

        test_parser(input, expected);
    }

    #[test]
    fn arrays_mixed_content() {
        let input = "[1, null, true, \"hello world\"]; [1, null, true, \"hello world\"]";

        let expected: Vec<Statement> = vec![];

        let expected = add_array_mixed_content(expected);
        let expected = add_array_mixed_content(expected);

        test_parser(input, expected);
    }

    #[test]
    fn array_index() {
        let input = "[0, 1][0]";

        let mut expected: Vec<Statement> = vec![];

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::from(parser::Expression::Index(Box::from(
                parser::expression::index::Index {
                    left: parser::Expression::Array(Box::from(Array {
                        values: vec![
                            Expression {
                                expression: Box::from(parser::Expression::Integer(Integer {
                                    value: 0,
                                })),
                            },
                            Expression {
                                expression: Box::from(parser::Expression::Integer(Integer {
                                    value: 1,
                                })),
                            },
                        ],
                    })),
                    index: parser::Expression::Integer(Integer { value: 0 }),
                },
            ))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn array_assign() {
        let input = "[0, 1][0] = 300";

        let mut expected: Vec<Statement> = vec![];

        expected.push(Statement::Expression(Box::from(Expression {
            expression: Box::from(parser::Expression::AssignIndex(Box::from(
                parser::expression::assign_index::AssignIndex {
                    left: parser::Expression::Array(Box::from(Array {
                        values: vec![
                            Expression {
                                expression: Box::from(parser::Expression::Integer(Integer {
                                    value: 0,
                                })),
                            },
                            Expression {
                                expression: Box::from(parser::Expression::Integer(Integer {
                                    value: 1,
                                })),
                            },
                        ],
                    })),
                    index: parser::Expression::Integer(Integer { value: 0 }),
                    value: parser::Expression::Integer(Integer { value: 300 }),
                },
            ))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn comments_single_line() {
        let input = "// Hello";

        let expected: Vec<Statement> = vec![];
        test_parser(input, expected);
    }

    #[test]
    fn comments_block() {
        let input = "/<hello \
        multiline >/";

        let expected: Vec<Statement> = vec![];
        test_parser(input, expected);
    }

    fn test_parser(input: &str, expected: Vec<Statement>) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                if let Exception::Syntax(err) = err {
                    println!("ParserException: {}", err);
                }
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
