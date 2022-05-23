#[cfg(test)]
mod tests {
    use crate::ast::instructions::function::{Call, Function};
    use crate::ast::instructions::memory::{CompoundType, Index, Load, LoadType, Push, Slice, Store};
    use crate::ast::instructions::suffix::{BinaryOperation, Suffix};
    use crate::ast::instructions::while_loop::While;
    use crate::ast::instructions::Node;
    use crate::ast::AST;
    use crate::lexer::token::Token;
    use crate::parser::Parser;
    use crate::types::{Type, ValueType};
    use logos::Logos;

    #[test]
    fn test_parser() {
        // 50
        let lexer = Token::lexer(".CONSTANT INT 50;.CONSTANT BOOL false;");
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::CONSTANT(ValueType::Integer(50)),
            Node::CONSTANT(ValueType::Boolean(false)),
        ]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_constants() {
        // 50; 2840; 5912;
        let lexer = Token::lexer(
            ".CONSTANT INT 50;.CONSTANT INT 2840;.CONSTANT INT 59123;.CONSTANT CHAR[] \"He\";",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::CONSTANT(ValueType::Integer(50)),
            Node::CONSTANT(ValueType::Integer(2840)),
            Node::CONSTANT(ValueType::Integer(59123)),
            Node::CONSTANT(ValueType::Array(Box::new(vec![
                ValueType::Character('H'),
                ValueType::Character('e'),
            ]))),
        ]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_compound() {
        let lexer = Token::lexer(
            ".COMPOUND \"TEST\" { INT; INT; CHAR[]; };",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::COMPOUND(CompoundType {
                name: "TEST".to_string(),
                values: Box::new(vec![Type::INT, Type::INT, Type::ARRAY(Box::new(Type::CHAR))])
            }),
        ]);

        assert!(parser.custom_types.get("TEST").is_some());

        let values = parser.custom_types.get("TEST").unwrap();
        assert_eq!(values[0], Type::INT);
        assert_eq!(values[1], Type::INT);
        assert_eq!(values[2], Type::ARRAY(Box::new(Type::CHAR)));


        assert_eq!(expected, result)

    }

    #[test]
    fn test_parser_compound_functions() {
        let lexer = Token::lexer(
            ".COMPOUND \"TEST\" { INT; INT; CHAR[]; }; \
            .FUNCTION \"\" 0 TEST ARGUMENTS { } FREE { } THEN { };"
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::COMPOUND(CompoundType {
                name: "TEST".to_string(),
                values: Box::new(vec![Type::INT, Type::INT, Type::ARRAY(Box::new(Type::CHAR))])
            }),
            Node::FUNCTION(Box::new(Function {
                name: "".to_string(),
                return_type: Type::Compound("TEST".to_string(), Box::new(vec![Type::INT, Type::INT, Type::ARRAY(Box::new(Type::CHAR))])),
                parameters: vec![],
                free: vec![],
                body: vec![],
                unique_identifier: 0
            }))
        ]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_compound_use() {
        let lexer = Token::lexer(
            ".COMPOUND \"TEST\" { INT; INT; CHAR[]; }; \
            .COMPOUND \"TEST_SECOND\" { INT; TEST; INT; }; \
            .CONSTANT TEST { .CONSTANT INT 10; .CONSTANT INT 40; .CONSTANT CHAR[] \"Hi\"; };\
            .CONSTANT TEST_SECOND { .CONSTANT INT 33; .CONSTANT TEST { .CONSTANT INT 10; .CONSTANT INT 40; .CONSTANT CHAR[] \"Hi\"; }; .CONSTANT INT 45; };",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::COMPOUND(CompoundType {
                name: "TEST".to_string(),
                values: Box::new(vec![Type::INT, Type::INT, Type::ARRAY(Box::new(Type::CHAR))])
            }),
            Node::COMPOUND(CompoundType {
                name: "TEST_SECOND".to_string(),
                values: Box::new(vec![Type::INT, Type::Compound("TEST".to_string(), Box::new(vec![Type::INT, Type::INT, Type::ARRAY(Box::new(Type::CHAR))])), Type::INT])
            }),
            Node::CONSTANT(ValueType::Compound("TEST".to_string(), Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(40),
                ValueType::Array(Box::new(vec![
                    ValueType::Character('H'),
                    ValueType::Character('i'),
                ]))
            ]))),
            Node::CONSTANT(ValueType::Compound("TEST_SECOND".to_string(), Box::new(vec![
                ValueType::Integer(33),
                ValueType::Compound("TEST".to_string(), Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(40),
                ValueType::Array(Box::new(vec![
                    ValueType::Character('H'),
                    ValueType::Character('i'),
                ]))
                ])),
                ValueType::Integer(45),
            ])))
        ]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_math() {
        // (50 + 2840) + 20
        let lexer = Token::lexer(".ADD { .CONSTANT INT 10; .CONSTANT INT 20; };");
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::SUFFIX(Box::new(Suffix {
            operation: BinaryOperation::ADD,
            left: Node::CONSTANT(ValueType::Integer(10)),
            right: Node::CONSTANT(ValueType::Integer(20)),
        }))]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_math_advanced_1() {
        let lexer = Token::lexer(
            ".ADD { .ADD { .CONSTANT INT 50; .CONSTANT INT 2840; }; .CONSTANT INT 20; };",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::SUFFIX(Box::new(Suffix {
            operation: BinaryOperation::ADD,
            left: Node::SUFFIX(Box::new(Suffix {
                operation: BinaryOperation::ADD,
                left: Node::CONSTANT(ValueType::Integer(50)),
                right: Node::CONSTANT(ValueType::Integer(2840)),
            })),
            right: Node::CONSTANT(ValueType::Integer(20)),
        }))]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_math_advanced_2() {
        let lexer = Token::lexer(
            "\n
        .ADD { \n
            .ADD { \n
                .CONSTANT INT 10; \n
                .MULTIPLY {\n
                    .CONSTANT INT 30;\n
                    .CONSTANT INT 50;\n
                }; \n
            }; \n
            .DIVIDE {\n
                .CONSTANT INT 10; \n
                .CONSTANT INT 5; \n
            }; \n
        };",
        );

        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::SUFFIX(Box::new(Suffix {
            operation: BinaryOperation::ADD,
            left: Node::SUFFIX(Box::new(Suffix {
                operation: BinaryOperation::ADD,
                left: Node::CONSTANT(ValueType::Integer(10)),
                right: Node::SUFFIX(Box::new(Suffix {
                    operation: BinaryOperation::MULTIPLY,
                    left: Node::CONSTANT(ValueType::Integer(30)),
                    right: Node::CONSTANT(ValueType::Integer(50)),
                })),
            })),
            right: Node::SUFFIX(Box::new(Suffix {
                operation: BinaryOperation::DIVIDE,
                left: Node::CONSTANT(ValueType::Integer(10)),
                right: Node::CONSTANT(ValueType::Integer(5)),
            })),
        }))]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_store() {
        // 50 + 2840 + 20
        let lexer = Token::lexer(
            "\
        .STORE 0 { .CONSTANT INT 50; };",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::STORE(Store {
            index: 0,
            value: Box::new(Node::CONSTANT(ValueType::Integer(50))),
        })]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_function() {
        let lexer = Token::lexer(".CALL {.FUNCTION \"\" 0 INT ARGUMENTS { INT; } FREE { } THEN { .ADD { .LOAD PARAMETER 0 0; .LOAD PARAMETER 0 0; }; };} { .CONSTANT INT 20; };");

        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::CALL(Box::new(Call {
            call: Node::FUNCTION(Box::new(Function {
                name: "".to_string(),
                return_type: Type::INT,
                parameters: vec![Type::INT],
                free: vec![],
                unique_identifier: 0,
                body: vec![Node::SUFFIX(Box::new(Suffix {
                    operation: BinaryOperation::ADD,
                    left: Node::LOAD(Load {
                        load_type: LoadType::PARAMETER(0),
                        index: 0,
                    }),
                    right: Node::LOAD(Load {
                        load_type: LoadType::PARAMETER(0),
                        index: 0,
                    }),
                }))],
            })),
            arguments: vec![Node::CONSTANT(ValueType::Integer(20))],
        }))]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parser_index() {
        // [10, 20][0]
        let lexer = Token::lexer(
            ".INDEX {.CONSTANT INT[] [.CONSTANT INT 10;.CONSTANT INT 20;];} {.CONSTANT INT 0;};",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::INDEX(Index {
            to_index: Box::new(Node::CONSTANT(ValueType::Array(Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(20),
            ])))),
            index: Box::new(Node::CONSTANT(ValueType::Integer(0))),
        })]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parser_push() {
        // [10, 20][0]
        let lexer = Token::lexer(
            ".PUSH {.CONSTANT INT[] [.CONSTANT INT 10;.CONSTANT INT 20;];} {.CONSTANT INT 0;};",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::PUSH(Push {
            to_push: Box::new(Node::CONSTANT(ValueType::Array(Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(20),
            ])))),
            item: Box::new(Node::CONSTANT(ValueType::Integer(0))),
        })]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parser_slice() {
        // [10, 20][0]
        let lexer = Token::lexer(
            ".SLICE {.CONSTANT INT[] [.CONSTANT INT 10;.CONSTANT INT 20;];} {.CONSTANT INT 0;} {.CONSTANT INT 1;};",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::SLICE(Slice {
            to_slice: Box::new(Node::CONSTANT(ValueType::Array(Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(20),
            ])))),
            from: Box::new(Node::CONSTANT(ValueType::Integer(0))),
            to: Box::new(Node::CONSTANT(ValueType::Integer(1))),
        })]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parser_array() {
        // [10, 20]
        let lexer = Token::lexer(".CONSTANT INT[] [.CONSTANT INT 10;.CONSTANT INT 20;];");
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::CONSTANT(ValueType::Array(Box::new(vec![
            ValueType::Integer(10),
            ValueType::Integer(20),
        ])))]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parser_array2d() {
        // [10, 20]
        let lexer = Token::lexer(".CONSTANT INT[][] [.CONSTANT INT[] [.CONSTANT INT 10;.CONSTANT INT 20;];.CONSTANT INT[] [.CONSTANT INT 30;.CONSTANT INT 40;];];");
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![Node::CONSTANT(ValueType::Array(Box::new(vec![
            ValueType::Array(Box::new(vec![
                ValueType::Integer(10),
                ValueType::Integer(20),
            ])),
            ValueType::Array(Box::new(vec![
                ValueType::Integer(30),
                ValueType::Integer(40),
            ])),
        ])))]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parser_store_load() {
        // variable = 50; variable + variable
        let lexer = Token::lexer(
            "\
        .STORE 0 { .CONSTANT INT 50; };\
        .ADD { .LOAD VARIABLE 0; .LOAD VARIABLE 0; };",
        );
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::STORE(Store {
                index: 0,
                value: Box::new(Node::CONSTANT(ValueType::Integer(50))),
            }),
            Node::SUFFIX(Box::new(Suffix {
                operation: BinaryOperation::ADD,
                left: Node::LOAD(Load {
                    load_type: LoadType::VARIABLE,
                    index: 0,
                }),
                right: Node::LOAD(Load {
                    load_type: LoadType::VARIABLE,
                    index: 0,
                }),
            })),
        ]);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_while() {
        let lexer = Token::lexer(".STORE 0 { .CONSTANT INT 0; }; .WHILE CONDITION { .GREATERTHAN { .CONSTANT INT 10; .LOAD VARIABLE 0; }; } THEN { .STORE 0 { .ADD { .LOAD VARIABLE 0; .CONSTANT INT 1; }; }; }; .LOAD VARIABLE 0;");

        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        let expected = AST::new_mock(vec![
            Node::STORE(Store {
                index: 0,
                value: Box::new(Node::CONSTANT(ValueType::Integer(0))),
            }),
            Node::WHILE(Box::new(While {
                condition: Node::SUFFIX(Box::new(Suffix {
                    operation: BinaryOperation::GREATERTHAN,
                    left: Node::CONSTANT(ValueType::Integer(10)),
                    right: Node::LOAD(Load {
                        load_type: LoadType::VARIABLE,
                        index: 0,
                    }),
                })),
                body: vec![Node::STORE(Store {
                    index: 0,
                    value: Box::new(Node::SUFFIX(Box::new(Suffix {
                        operation: BinaryOperation::ADD,
                        left: Node::LOAD(Load {
                            load_type: LoadType::VARIABLE,
                            index: 0,
                        }),
                        right: Node::CONSTANT(ValueType::Integer(1)),
                    }))),
                })],
            })),
            Node::LOAD(Load {
                load_type: LoadType::VARIABLE,
                index: 0,
            }),
        ]);

        assert_eq!(expected, result)
    }
}
