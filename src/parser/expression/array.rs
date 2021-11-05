use crate::lexer::token::TokenType;
use crate::parser;
use crate::parser::expression::Precedence::Lowest;
use crate::parser::program::Node;
use crate::parser::statement::expression::Expression;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub(crate) values: Vec<Expression>,
}

pub fn parse_expression_array(p: &mut Parser) -> Option<Node> {
    let mut elements: Vec<Expression> = Vec::new();

    p.lexer.next_token();

    while p.lexer.current_token.clone().unwrap().token != TokenType::RightBracket
        && p.lexer.current_token.clone().unwrap().token != TokenType::Eof
    {
        let exp = p.parse_expression(Lowest);

        if let Some(Node::Expression(exp)) = exp {
            elements.push(Expression {
                expression: Box::from(exp),
            });
        }

        p.lexer.next_token();

        if p.lexer.current_token.clone().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    Some(Node::Expression(crate::parser::Expression::Array(
        Box::from(Array { values: elements }),
    )))
}
