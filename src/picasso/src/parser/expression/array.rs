use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::Precedence::Lowest;
use crate::parser::program::Node;
use crate::parser::statement::expression::Expression;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
// TODO: Add type
pub struct Array {
    pub(crate) values: Vec<Expression>,
    // Line, colon
    pub location: (i32, i32),
}

pub fn parse_expression_array(p: &mut Parser) -> Result<Node, SyntaxException> {
    let mut elements: Vec<Expression> = Vec::new();

    p.lexer.next_token();

    while p.lexer.get_current_token().unwrap().token != TokenType::RightBracket
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        let exp = p.parse_expression(Lowest);

        if let Ok(Node::Expression(exp)) = exp {
            elements.push(Expression {
                expression: Box::from(exp),
            });
        }

        p.lexer.next_token();

        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    Ok(Node::Expression(crate::parser::Expression::Array(
        Box::from(Array {
            values: elements,
            location: (p.lexer.current_line, p.lexer.current_col),
        }),
    )))
}
