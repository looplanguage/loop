use crate::parser;
use crate::parser::statement;
use crate::parser::expression;
use crate::parser::statement::Statement;
use crate::parser::statement::expression::Expression;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::integer::Integer;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::suffix::Suffix;
use crate::parser::statement::block::Block;
use crate::parser::program::Node;
use crate::parser::expression::conditional::Conditional;

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