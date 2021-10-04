use crate::lexer;
use crate::parser;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::statement::Statement;
use std::borrow::Borrow;
use crate::parser::expression::suffix::Suffix;

#[cfg(test)]

#[test]
fn variable_declaration() {
    let input =
        "var test = 1;
        var test2 = 40;
        var test3 = 10 * 2;
        ";

    let mut expected: Vec<Statement> = Vec::new();

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test".to_string(),
        },
        value: Expression::Integer(Integer { value: 1 }),
    }));

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test2".to_string(),
        },
        value: Expression::Integer(Integer { value: 40 }),
    }));

    expected.push(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: "test3".to_string(),
        },
        value: Expression::Suffix(Box::new(Suffix{
            left: Expression::Integer(Integer{ value: 10}),
            operator: '*',
            right: Expression::Integer(Integer{ value: 2 })
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
