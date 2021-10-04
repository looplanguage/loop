#![cfg(test)]
use crate::lexer;
use crate::parser;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::suffix::Suffix;
use crate::parser::statement::expression::Expression;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::statement::Statement;

#[test]
fn functions() {
    let input = "true; false;";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: true }),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: false }),
    }));

    test_parser(input, expected);
}

#[test]
fn booleans() {
    let input = "true; false;";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: true }),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: false }),
    }));

    test_parser(input, expected);
}

#[test]
fn booleans_inverted() {
    let input = "!true; !false;";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: false }),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Boolean(Boolean { value: true }),
    }));

    test_parser(input, expected);
}

#[test]
fn comparison() {
    let input = "true == true; 1 == true; 3 > 4; 3 < 4;";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Suffix(Box::new(Suffix{
            left: parser::expression::Expression::Boolean(Boolean{ value: true }),
            operator: "==".to_string(),
            right: parser::expression::Expression::Boolean(Boolean{ value: true }),
        })),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Suffix(Box::new(Suffix{
            left: parser::expression::Expression::Integer(Integer{ value: 1 }),
            operator: "==".to_string(),
            right: parser::expression::Expression::Boolean(Boolean{ value: true }),
        })),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Suffix(Box::new(Suffix{
            left: parser::expression::Expression::Integer(Integer{ value: 3 }),
            operator: ">".to_string(),
            right: parser::expression::Expression::Integer(Integer{ value: 4 }),
        })),
    }));

    expected.push(Statement::Expression(Expression {
        expression: parser::expression::Expression::Suffix(Box::new(Suffix{
            left: parser::expression::Expression::Integer(Integer{ value: 3 }),
            operator: "<".to_string(),
            right: parser::expression::Expression::Integer(Integer{ value: 4 }),
        })),
    }));

    test_parser(input, expected);
}

#[test]
fn variable_declaration() {
    let input = "var test = 1;
        var test2 = 40;
        var test3 = 10 * 2;
        ";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test".to_string(),
        },
        value: parser::expression::Expression::Integer(Integer { value: 1 }),
    }));

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test2".to_string(),
        },
        value: parser::expression::Expression::Integer(Integer { value: 40 }),
    }));

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test3".to_string(),
        },
        value: parser::expression::Expression::Suffix(Box::new(Suffix {
            left: parser::expression::Expression::Integer(Integer { value: 10 }),
            operator: '*'.to_string(),
            right: parser::expression::Expression::Integer(Integer { value: 2 }),
        })),
    }));

    test_parser(input, expected);
}

fn test_parser(input: &str, expected: Vec<Statement>) {
    let l = lexer::build_lexer(input);
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    let mut i = 0;
    for statement in program.statements {
        assert_eq!(statement, expected[i]);

        i += 1;
    }
}
