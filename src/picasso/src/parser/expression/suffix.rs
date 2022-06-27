use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::Expression;
use crate::parser::expression::Precedence::Lowest;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Suffix {
    pub(crate) left: Expression,
    pub(crate) operator: String,
    pub(crate) right: Expression,
}

pub fn parse_suffix_expression(p: &mut Parser, left: Expression) -> Result<Node, SyntaxException> {
    let operator = p.lexer.get_current_token().unwrap().literal.clone();

    let pre = p.current_precedence();

    p.lexer.next_token();

    let exp = p.parse_expression(pre)?;

    if let Node::Expression(val) = exp {
        return Ok(Node::Expression(Expression::Suffix(Box::new(Suffix {
            left,
            operator,
            right: val,
        }))));
    }

    Err(SyntaxException::Unknown)
}

pub fn parse_grouped_expression(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer.next_token();
    let exp = p.parse_expression(Lowest)?;

    p.lexer.next_token();

    p.current_token_is_result(TokenType::RightParenthesis)?;

    Ok(exp)
}

/// Same as: `parse_grouped_expression`, except it is for for and if expressions, without any parenthesis
///
/// Returns: `Option<Node>`
pub fn parse_grouped_expression_without_param(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer.next_token();

    Ok(p.parse_expression(Lowest)?)
}
