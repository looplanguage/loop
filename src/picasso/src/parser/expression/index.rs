use crate::lexer::token::TokenType;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::parse_call;
use crate::parser::expression::function::parse_expression_arguments;
use crate::parser::expression::identifier::parse_identifier;
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
        // TODO: This causes extension methods to break, as they start with an identifier as well.
    } else if p.lexer.get_current_token().unwrap().token == TokenType::Identifier {
        let y = p.lexer.get_current_token().unwrap().clone().literal;

        if p.lexer.get_peek_token().unwrap().clone().token != TokenType::LeftParenthesis {
            if p.lexer.next_token_is_and_next_token(TokenType::Assign) {
                p.lexer.next_token();

                let value = p.parse_expression(Precedence::Lowest);

                if let Node::Expression(exp) = value.unwrap() {
                    return Some(Node::Expression(Expression::AssignIndex(Box::from(
                        AssignIndex {
                            left,
                            index: Expression::Identifier(Identifier {
                                value: y.clone()
                            }),
                            value: exp,
                        },
                    ))));
                }
            }

            return Some(Node::Expression(Expression::Index(Box::new(Index {
                left,
                index: Expression::Identifier(Identifier {
                    value: y.clone()
                })
            }))))
        }

        p.lexer.next_token();
        let arguments: Vec<Expression> = parse_expression_arguments(p);

        if !p.current_token_is(TokenType::RightParenthesis) {
            p.add_error(format!(
                "wrong token. got=\"{:?}\". expected=\"RightParenthesis\"",
                p.lexer.get_current_token().unwrap().token
            ));
            return None;
        };

        let x = if let Expression::Identifier(i) = left {
            i.value
        } else {
            unreachable!()
        };

        let identifier = Expression::Identifier(Identifier {
            value: format!("{}::{}", x, y),
        });

        return Some(Node::Expression(Expression::Call(Call {
            identifier: Box::from(identifier),
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
