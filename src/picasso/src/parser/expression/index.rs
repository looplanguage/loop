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

#[derive(Debug, PartialEq, Clone)]
pub struct Slice {
    pub left: Box<Expression>,
    pub begin: Box<Expression>,
    pub end: Box<Expression>,
}

impl Slice {
    pub fn new_node(left: Expression, begin: Expression, end: Expression) -> Expression {
        Expression::Slice(Slice {
            left: Box::new(left),
            begin: Box::new(begin),
            end: Box::new(end),
        })
    }
}

pub fn parse_index_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    let check_token = p.lexer.get_current_token().unwrap().token;
    p.lexer.next_token(); // Skipping over potential left bracket

    if check_token == TokenType::LeftBracket {
        // This index expression is for: Arrays OR Hashmaps
        let index_exp = p.parse_expression(Precedence::Lowest);

        if p.lexer.peek_token.as_ref().unwrap().token == TokenType::DotDot {
            // Is slice and not an index

            p.lexer.next_token(); // Skipping over dotdot
            p.lexer.next_token(); // Skipping over end expression

            // 'end' is the right value of the slice
            let end = p.parse_expression(Precedence::Lowest);

            let begin = if let Node::Expression(begin) = index_exp.unwrap() {
                begin
            } else {
                // TODO: Create default value so this is possible: arr[..4]
                todo!("Create error for when begin is not expression")
            };

            let end = if let Node::Expression(end) = end.unwrap() {
                end
            } else {
                // TODO: Create default value so this is possible: arr[1..]
                todo!("Create error for when end is not expression")
            };

            return Some(Node::Expression(Slice::new_node(left, begin, end)));
        }

        if let Node::Expression(index) = index_exp.unwrap() {
            p.lexer.next_token(); // Skipping over expression

            // Now we check if we want to assign to this index, otherwise just return the index
            if p.peek_token_is(TokenType::Assign) {
                p.lexer.next_token(); // Assign
                p.lexer.next_token(); // Skipping over value

                // Parsing of value
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

        let namespace = match left {
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

        // Index & Assign
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
