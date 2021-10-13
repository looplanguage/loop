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
use crate::parser::expression::function::Function;
use std::process::id;

// ========================================================================
// Everything with "v3" behind the identifier are newer functions for the refactor
// ========================================================================

pub fn generate_variable_declaration_v3(identifier: &str, expression: parser::expression::Expression) -> Statement {
    let variable = Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: identifier.to_string(),
        },
        value: Box::new(expression),
    });
    return variable;
}

pub fn generate_suffix_expression_v3(left: parser::expression::Expression, operator: &str, right: parser::expression::Expression) -> crate::parser::expression::Expression {
    parser::expression::Expression::Suffix(Box::new(Suffix {
        left: left,
        operator: operator.to_string(),
        right: right,
    }))
}

pub fn generate_comparison_v3(left: parser::expression::Expression, operator: &str, right: parser::expression::Expression) -> Statement {
    Statement::Expression(Box::new(Expression {
        expression: Box::new(parser::expression::Expression::Suffix(Box::new(Suffix {
            left: left,
            operator: operator.to_string(),
            right: right,
        }))),
    }))
}

pub fn generate_function_v3_box(parameters: Vec<Identifier>, statements: Vec<Statement>) -> Box<crate::parser::expression::Expression> {
    Box::new(parser::expression::Expression::Function(Function {
        parameters: parameters,
        body: Block { statements: statements },
    }))
}

pub fn generate_function_v3(parameters: Vec<Identifier>, statements: Vec<Statement>) -> crate::parser::expression::Expression {
    parser::expression::Expression::Function(Function {
        parameters: parameters,
        body: Block { statements: statements },
    })
}

pub fn generate_expression_statement_v3(expression: parser::expression::Expression) -> Statement {
    Statement::Expression(Box::new(Expression {
        expression: Box::new(expression),
    }))
}

pub fn generate_identifier_expression_v3(identifier: &str) -> crate::parser::expression::Expression {
    parser::expression::Expression::Identifier(Identifier {
        value: identifier.to_string(),
    })
}

pub fn generate_identifier_v3(name: &str) -> Identifier {
    Identifier {
        value: name.to_string(),
    }
}


// ====================================================================
// Everything underneath here is things with if expression parsing test
// ToDo: Will be refactored
//====================================================================

pub fn generate_boolean_expression_box(value: bool) -> Statement {
    let expression = Statement::Expression(Box::new(Expression {
        expression: Box::new(generate_boolean_expression(value)),
    }));
    return expression;
}

pub fn generate_boolean_expression(value: bool) -> crate::parser::expression::Expression {
    return parser::expression::Expression::Boolean(Boolean {
        value: value,
    });
}

pub fn generate_integer_expression_box(value: i32) -> Statement {
    let expression = Statement::Expression(Box::new(Expression {
        expression: Box::new(generate_integer_expression(value)),
    }));
    return expression;
}

pub fn generate_integer_expression(value: i32) -> crate::parser::expression::Expression {
    return parser::expression::Expression::Integer(Integer {
        value: value,
    });
}

pub fn generate_expression_suffix(left: i32, operator: char, right: i32) -> crate::parser::expression::Expression {
    let suffix_expression =parser::expression::Expression::Suffix(Box::new(Suffix {
        left: parser::expression::Expression::Integer(Integer { value: left }),
        operator: operator.to_string(),
        right: parser::expression::Expression::Integer(Integer { value: right }),
    }));
    return suffix_expression;
}

pub fn generate_if_expression(condition: bool, body: Block, else_condition: Box<Option<Node>>) -> Statement {
    return Statement::Expression(Box::new(Expression {
        expression: Box::new(parser::Expression::Conditional(Box::new(generate_conditional(condition, body, else_condition)))),
    }));
}

pub fn generate_else_condition(conditinional: bool, body: Block, else_condition: Box<Option<Node>>) -> Box<Option<Node>> {
    return Box::new(Some(parser::Node::Expression(
        parser::expression::Expression::Conditional(Box::new(generate_conditional(conditinional, body, else_condition))),
    )));
}

pub fn generate_conditional(condition: bool, body: Block, else_condition: Box<Option<Node>>) -> Conditional {
    return Conditional {
        condition: Box::new(parser::Expression::Boolean(Boolean { value: condition })),
        body: body,
        else_condition: else_condition,
    }
}

pub fn generate_else_block_box(statements: Vec<Statement>) -> Box<Option<Node>> {
    return Box::new(Some(parser::Node::Statement(Statement::Block(
        generate_else_block(statements),
    ))));
}

pub fn generate_else_block(statements: Vec<Statement>) -> Block {
    return Block { statements: statements };
}