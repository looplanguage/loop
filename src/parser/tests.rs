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
    use crate::parser::program::Node;

    fn generate_if_expression(condition: bool, body: Block, else_condition: Box<Option<Node>>) -> Statement {
        return Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::Expression::Conditional(Box::new(generate_conditional(condition, body, else_condition)))),
        }));
    }

    fn generate_else_condition(conditinional: bool, body: Block, else_condition: Box<Option<Node>>) -> Box<Option<Node>> {
        return Box::new(Some(parser::Node::Expression(
            parser::expression::Expression::Conditional(Box::new(generate_conditional(conditinional, body, else_condition))),
        )));
    }

    fn generate_conditional(condition: bool, body: Block, else_condition: Box<Option<Node>>) -> Conditional {
        return Conditional {
            condition: Box::new(parser::Expression::Boolean(Boolean { value: condition })),
            body: body,
            else_condition: else_condition,
        }
    }

    fn generate_else_block_box(statements: Vec<Statement>) -> Box<Option<Node>> {
       return Box::new(Some(parser::Node::Statement(Statement::Block(
           generate_else_block(statements),
        ))));
    }

    fn generate_else_block(statements: Vec<Statement>) -> Block {
        return Block { statements: statements };
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

        expected.push(generate_if_expression(true, Block { statements: vec![] }, Box::new(None)));


        let else_conditional = generate_else_condition(true, Block { statements: vec![] }, Box::new(None));
        expected.push(generate_if_expression(false, Block { statements: vec![] }, else_conditional));


        let else_conditional = generate_else_condition(false, Block { statements: vec![] }, generate_else_block_box(vec![]));
        expected.push(generate_if_expression(false, Block { statements: vec![] }, else_conditional));

        // TODO: This is a hot mess. It is kind of readable but not really...
        let else_conditional = generate_else_condition(false, generate_else_block(vec![generate_boolean_expression_box(true), Statement::Expression(Box::new(Expression {
            expression: Box::new(generate_expression_suffix(1, '+', 1)),
        }))]), generate_else_block_box(vec![Statement::Expression(Box::new(Expression {
            expression: Box::new(generate_boolean_expression(true)),
        }))]));

        expected.push(generate_if_expression(false, Block {
            statements: vec![
                generate_integer_expression_box(1),
                generate_boolean_expression_box(true),
            ],
        }, else_conditional));

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

        expected.push(generate_boolean_expression_box(true));
        expected.push(generate_boolean_expression_box(false));

        test_parser(input, expected);
    }

    #[test]
    fn booleans_inverted() {
        let input = "!true; !false;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(generate_boolean_expression_box(false));
        expected.push(generate_boolean_expression_box(true));

        test_parser(input, expected);
    }

    #[test]
    fn comparison() {
        let input = "true == true; 1 == true; 3 > 4; 3 < 4;";

        let mut expected: Vec<Statement> = Vec::new();

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: generate_boolean_expression(true),
                operator: generate_operator("=="),
                right: generate_boolean_expression(true),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: generate_integer_expression(1),
                operator: generate_operator("=="),
                right: generate_boolean_expression(true),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: generate_integer_expression(3),
                operator: generate_operator(">"),
                right: generate_integer_expression(4),
            }))),
        })));

        expected.push(Statement::Expression(Box::new(Expression {
            expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
                left: generate_integer_expression(3),
                operator: generate_operator("<"),
                right: generate_integer_expression(4),
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

        expected.push(generate_variable_declaration("test", 1));
        expected.push(generate_variable_declaration("test2", 40));
        expected.push(generate_variable_declaration_suffix("test3", generate_expression_suffix(10, '*', 2)));

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

    // Helper Functions

    fn generate_operator(operator: &str) -> String {
        return operator.to_string();
    }

    fn generate_boolean_expression_box(value: bool) -> Statement {
        let expression = Statement::Expression(Box::new(Expression {
            expression: Box::new(generate_boolean_expression(value)),
        }));
        return expression;
    }

    fn generate_boolean_expression(value: bool) -> crate::parser::expression::Expression {
        return parser::expression::Expression::Boolean(Boolean {
            value: value,
        });
    }

    fn generate_integer_expression_box(value: i32) -> Statement {
        let expression = Statement::Expression(Box::new(Expression {
            expression: Box::new(generate_integer_expression(value)),
        }));
        return expression;
    }

    fn generate_integer_expression(value: i32) -> crate::parser::expression::Expression {
        return parser::expression::Expression::Integer(Integer {
            value: value,
        });
    }

    fn generate_variable_declaration(identifier: &str, expression: i32) -> Statement {
        let variable = Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: identifier.to_string(),
            },
            value: Box::new(parser::expression::Expression::Integer(Integer {
                value: expression,
            })),
        });
        return variable;
    }

    fn generate_variable_declaration_suffix(identifier: &str, Suffix: crate::parser::expression::Expression) -> Statement {
        let variable = Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: identifier.to_string(),
            },
            value: Box::new(Suffix),
        });
        return variable;
    }

    fn generate_expression_suffix(left: i32, operator: char, right: i32) -> crate::parser::expression::Expression {
        let suffix_expression =parser::expression::Expression::Suffix(Box::new(Suffix {
            left: parser::expression::Expression::Integer(Integer { value: left }),
            operator: operator.to_string(),
            right: parser::expression::Expression::Integer(Integer { value: right }),
        }));
        return suffix_expression;
    }
}
