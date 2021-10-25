use crate::lexer::token::TokenType;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::parse_call;
use crate::parser::expression::identifier::parse_identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Index {
    pub(crate) left: Expression,
    pub(crate) index: Expression,
}

pub fn parse_index_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    if p.lexer.current_token.clone().unwrap().token == TokenType::LeftBracket {
        // This index expression is for: Arrays OR Hashmaps
        p.lexer.next_token();

        let index_exp = p.parse_expression(Precedence::Lowest);

        if let Node::Expression(index) = index_exp.unwrap() {
            p.lexer.next_token();

            // Now we check if we want to assign to this index, otherwise just return the index
            if p.lexer.next_is(TokenType::Assign) {
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
    } else {
        // This index expression is for: Extension methods OR Hashmaps
        p.lexer.next_token();

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
