use crate::lexer::token::TokenType;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::parse_call;
use crate::parser::expression::function::parse_expression_arguments;
use crate::parser::expression::identifier::parse_identifier;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::Call;
use crate::parser::expression::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Index {
    pub(crate) left: Expression,
    pub(crate) index: Expression,
}

pub fn parse_index_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    let check_token = p.lexer.get_current_token().unwrap().token;
    p.lexer.next_token();

    if check_token == TokenType::LeftBracket {
        // This index expression is for: Arrays OR Hashmaps

        let index_exp = p.parse_expression(Precedence::Lowest);

        if let Node::Expression(index) = index_exp.unwrap() {
            p.lexer.next_token();

            // Now we check if we want to assign to this index, otherwise just return the index
            if p.lexer.next_token_is_and_next_token(TokenType::Assign) {
                p.lexer.next_token();

                let value = p.parse_expression(Precedence::Lowest);

                if let Node::Expression(exp) = value.unwrap() {
                    return Some(Node::Expression(Expression::AssignIndex(Box::from(
                        AssignIndex {
                            left,
                            index,
                            value: exp,
                        },
                    ))));
                }
            }

            return Some(Node::Expression(Expression::Index(Box::from(Index {
                left,
                index,
            }))));
        }
    } else if p.lexer.get_current_token().unwrap().token == TokenType::Identifier {
        let identifier = p.lexer.get_current_token().unwrap().clone().literal;

        if p.lexer.get_peek_token().unwrap().clone().token != TokenType::LeftParenthesis {
            if p.lexer.next_token_is_and_next_token(TokenType::Assign) {
                p.lexer.next_token();

                let value = p.parse_expression(Precedence::Lowest);

                if let Node::Expression(exp) = value.unwrap() {
                    return Some(Node::Expression(Expression::AssignIndex(Box::from(
                        AssignIndex {
                            left,
                            index: Expression::Identifier(Identifier { value: identifier }),
                            value: exp,
                        },
                    ))));
                }
            }
            return Some(Node::Expression(Expression::Index(Box::new(Index {
                left,
                index: Expression::Identifier(Identifier { value: identifier }),
            }))));
        }

        p.expected(TokenType::LeftParenthesis)?;
        let arguments: Vec<Expression> = parse_expression_arguments(p);

        let namespace = match left.clone() {
            Expression::Identifier(i) => i.value,
            _ => {
                return Some(Node::Expression(Expression::Call(Call {
                    identifier: Box::from(Expression::Index(Box::new(Index {
                        left: left.clone(),
                        index: Expression::Identifier(Identifier { value: identifier }),
                    }))),
                    parameters: arguments,
                })));
            }
        };

        match &*identifier {
            "add" | "remove" => {
                return Some(Node::Expression(Expression::Call(Call {
                    identifier: Box::new(Expression::Index(Box::new(Index {
                        left,
                        index: Expression::Identifier(Identifier {
                            value: identifier
                        })
                    }))),
                    parameters: arguments
                })))
            }
            _ => {}
        }

        // Index **AND** Assign
        return Some(Node::Expression(Expression::Call(Call {
            identifier: Box::from(Expression::String(LoopString {
                value: format!("{}::{}", namespace, identifier),
            })),
            parameters: arguments,
        })));
    } else {
        // This index expression is for: Extension methods OR Classes
        let identifier = parse_identifier(p);
        if let Node::Expression(ident_exp) = identifier.unwrap() {
            p.lexer.next_token();
            let exp = parse_call(p, ident_exp);

            exp.as_ref()?;

            if let Node::Expression(val) = exp.unwrap() {
                return Some(Node::Expression(Expression::Index(Box::from(Index {
                    left,
                    index: val,
                }))));
            }
        }
    }

    None
}
