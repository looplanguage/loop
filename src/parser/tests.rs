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
    use crate::parser::statement::block::Block;
    use crate::parser::statement::expression::Expression;
    use crate::parser::statement::variable::VariableDeclaration;
    use crate::parser::statement::Statement;

    #[test]
    fn conditionals() {
        let input = "\
        if(true) {} \
        if(false) {} else if(true) {}\
        if(false) {} else if(false) {} else {}\
        if(false) { 1; true; } else if(false) {true; 1 + 1} else { true; }\
        ";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Conditional(Box::new(Conditional {
                condition: Box::new(parser::Expression::Boolean(Boolean { value: true })),
                body: Block { statements: vec![] },
                else_condition: Box::new(None),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Conditional(Box::new(Conditional {
                condition: Box::new(parser::Expression::Boolean(Boolean { value: false })),
                body: Block { statements: vec![] },
                else_condition: Box::new(Some(parser::Node::Expression(
                    parser::expression::Expression::Conditional(Box::new(Conditional {
                        condition: Box::new(parser::Expression::Boolean(Boolean { value: true })),
                        body: Block { statements: vec![] },
                        else_condition: Box::new(None),
                    })),
                ))),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Conditional(Box::new(Conditional {
                condition: Box::new(parser::Expression::Boolean(Boolean { value: false })),
                body: Block { statements: vec![] },
                else_condition: Box::new(Some(parser::Node::Expression(
                    parser::expression::Expression::Conditional(Box::new(Conditional {
                        condition: Box::new(parser::Expression::Boolean(Boolean { value: false })),
                        body: Block { statements: vec![] },
                        else_condition: Box::new(Some(parser::Node::Statement(Statement::Block(
                            Block { statements: vec![] },
                        )))),
                    })),
                ))),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Conditional(Box::new(Conditional {
                condition: Box::new(parser::Expression::Boolean(Boolean { value: false })),
                body: Block {
                    statements: vec![Statement::Expression(Box::new(Expression {
                        expression: Box::new(parser::expression::Expression::Integer(Integer {
                            value: 1,
                        })),
                    })), Statement::Expression(Box::new(Expression {
                        expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                            value: true,
                        })),
                    }))],
                },
                else_condition: Box::new(Some(parser::Node::Expression(
                    parser::expression::Expression::Conditional(Box::new(Conditional {
                        condition: Box::new(parser::Expression::Boolean(Boolean { value: false })),
                        body: Block { statements: vec![
                            Statement::Expression(Box::new(Expression {
                                expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                                    value: true,
                                })),
                            })), Statement::Expression(Box::new(Expression {
                                expression: Box::new(parser::expression::Expression::Suffix(Box::from(Suffix {
                                    left: parser::expression::Expression::Integer(Integer{ value : 1}),
                                    operator: "+".to_string(),
                                    right: parser::expression::Expression::Integer(Integer{ value : 1})
                                }))),
                            }))
                        ] },
                        else_condition: Box::new(Some(parser::Node::Statement(Statement::Block(
                            Block { statements: vec![Statement::Expression(Box::new(Expression {
                                expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                                    value: true,
                                })),
                            }))] },
                        )))),
                    })),
                ))),
            }))),
        })));

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

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Function(Function {
                parameters: vec![],
                body: Block { statements: vec![] },
            })),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Function(Function {
                parameters: vec![Identifier {
                    value: "a".to_string(),
                }],
                body: Block { statements: vec![] },
            })),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Function(Function {
                parameters: vec![
                    Identifier {
                        value: "a".to_string(),
                    },
                    Identifier {
                        value: "b".to_string(),
                    },
                    Identifier {
                        value: "c".to_string(),
                    },
                    Identifier {
                        value: "d".to_string(),
                    },
                ],
                body: Block { statements: vec![] },
            })),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Function(Function {
                parameters: vec![],
                body: Block {
                    statements: vec![Statement::Expression(Box::new(Expression {
                        expression: Box::new(parser::expression::Expression::Integer(Integer {
                            value: 1,
                        })),
                    }))],
                },
            })),
        })));

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "functionWithParameters".to_string(),
            },
            value: Box::new(parser::expression::Expression::Function(Function {
                parameters: vec![
                    Identifier {
                        value: "a".to_string(),
                    },
                    Identifier {
                        value: "b".to_string(),
                    },
                    Identifier {
                        value: "c".to_string(),
                    },
                    Identifier {
                        value: "d".to_string(),
                    },
                ],
                body: Block {
                    statements: vec![
                        Statement::Expression(Box::new(Expression {
                            expression: Box::new(parser::expression::Expression::Suffix(Box::new(
                                Suffix {
                                    left: parser::expression::Expression::Identifier(Identifier {
                                        value: "a".to_string(),
                                    }),
                                    operator: "+".to_string(),
                                    right: parser::expression::Expression::Identifier(Identifier {
                                        value: "b".to_string(),
                                    }),
                                },
                            ))),
                        })),
                        Statement::VariableDeclaration(VariableDeclaration {
                            ident: Identifier {
                                value: "e".to_string(),
                            },
                            value: Box::new(parser::expression::Expression::Suffix(Box::new(
                                Suffix {
                                    left: parser::expression::Expression::Identifier(Identifier {
                                        value: "c".to_string(),
                                    }),
                                    operator: "+".to_string(),
                                    right: parser::expression::Expression::Identifier(Identifier {
                                        value: "d".to_string(),
                                    }),
                                },
                            ))),
                        }),
                    ],
                },
            })),
        }));

        test_parser(input, expected);
    }

    #[test]
    fn booleans() {
        let input = "true; false;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                value: true,
            })),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                value: false,
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn booleans_inverted() {
        let input = "!true; !false;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                value: false,
            })),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Boolean(Boolean {
                value: true,
            })),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn comparison() {
        let input = "true == true; 1 == true; 3 > 4; 3 < 4;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: parser::expression::Expression::Boolean(Boolean { value: true }),
                operator: "==".to_string(),
                right: parser::expression::Expression::Boolean(Boolean { value: true }),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: parser::expression::Expression::Integer(Integer { value: 1 }),
                operator: "==".to_string(),
                right: parser::expression::Expression::Boolean(Boolean { value: true }),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: parser::expression::Expression::Integer(Integer { value: 3 }),
                operator: ">".to_string(),
                right: parser::expression::Expression::Integer(Integer { value: 4 }),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: parser::expression::Expression::Integer(Integer { value: 3 }),
                operator: "<".to_string(),
                right: parser::expression::Expression::Integer(Integer { value: 4 }),
            }))),
        })));

        test_parser(input, expected);
    }

    #[test]
    fn variable_declaration() {
        let input = "var test = 1;
        var test2 = 40;
        var test3 = 10 * 2;
        ";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "test".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 1,
            })),
        }));

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "test2".to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: 40,
            })),
        }));

        expected.push(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: "test3".to_string(),
            },
            value: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: parser::expression::Expression::Integer(Integer { value: 10 }),
                operator: '*'.to_string(),
                right: parser::expression::Expression::Integer(Integer { value: 2 }),
            }))),
        }));

        test_parser(input, expected);
    }

    fn test_parser(input: &str, expected: Vec<Statement>) {
        let l = lexer::build_lexer(input);
        let mut parser = parser::build_parser(l);

        let program = parser.parse();

        let mut i = 0;
        for statement in program.statements {
            assert_eq!(statement, expected[i]);

            i += 1;
        }
    }
}
