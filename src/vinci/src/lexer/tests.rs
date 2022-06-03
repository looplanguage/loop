#[cfg(test)]
mod tests {
    use crate::ast::instructions::memory::LoadType;
    use crate::lexer::token::{Instruction, Token};
    use crate::types::Type;
    use logos::Logos;

    #[test]
    fn lexer_next() {
        let mut lexer = Token::lexer(".CONSTANT INT 50");

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::CONSTANT)
        );
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
        assert_eq!(lexer.next().unwrap(), Token::Number(50));
    }

    #[test]
    fn lexer_array() {
        let mut lexer = Token::lexer("INT[][][] [10,20,30]");

        assert_eq!(
            lexer.next().unwrap(),
            Token::Type(Type::ARRAY(Box::new(Type::ARRAY(Box::new(Type::ARRAY(
                Box::new(Type::INT)
            ))))))
        );

        assert_eq!(lexer.next().unwrap(), Token::LeftBracket);
        assert_eq!(lexer.next().unwrap(), Token::Number(10));
        assert_eq!(lexer.next().unwrap(), Token::Comma);
        assert_eq!(lexer.next().unwrap(), Token::Number(20));
        assert_eq!(lexer.next().unwrap(), Token::Comma);
        assert_eq!(lexer.next().unwrap(), Token::Number(30));
        assert_eq!(lexer.next().unwrap(), Token::RightBracket);
    }

    #[test]
    fn strings() {
        let mut lexer = Token::lexer("\"Hello World!\"");

        assert_eq!(
            lexer.next().unwrap(),
            Token::String(vec![
                'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!',
            ])
        )
    }

    #[test]
    fn characters() {
        let mut lexer = Token::lexer("'a' 'b'");

        assert_eq!(lexer.next().unwrap(), Token::Character('a'));
        assert_eq!(lexer.next().unwrap(), Token::Character('b'))
    }

    #[test]
    fn lexer_load_type() {
        let mut lexer = Token::lexer("PARAMETER");

        assert_eq!(
            lexer.next().unwrap(),
            Token::LoadType(LoadType::PARAMETER(0))
        );
    }

    #[test]
    fn lexer_float() {
        let mut lexer = Token::lexer("FLOAT");

        assert_eq!(lexer.next().unwrap(), Token::Type(Type::FLOAT));
    }

    #[test]
    fn lexer_namespace() {
        let mut lexer = Token::lexer("local::0");

        assert_eq!(
            lexer.next().unwrap(),
            Token::Namespace("local::0".to_string())
        );
    }

    #[test]
    fn lexer_multi() {
        let mut lexer = Token::lexer(".CONSTANT INT 50;.CONSTANT INT 500");

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::CONSTANT)
        );
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
        assert_eq!(lexer.next().unwrap(), Token::Number(50));

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::CONSTANT)
        );
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
        assert_eq!(lexer.next().unwrap(), Token::Number(500));
    }

    #[test]
    fn lexer_multi_newline() {
        let mut lexer = Token::lexer(
            ".CONSTANT INT 50;\
        \n.CONSTANT INT 500",
        );

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::CONSTANT)
        );
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
        assert_eq!(lexer.next().unwrap(), Token::Number(50));

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::CONSTANT)
        );
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
        assert_eq!(lexer.next().unwrap(), Token::Number(500));
    }

    #[test]
    fn lexer_no_spaces() {
        let mut lexer = Token::lexer(".FUNCTION 0 ARGUMENTS { INT; }");

        assert_eq!(
            lexer.next().unwrap(),
            Token::Instruction(Instruction::FUNCTION)
        );
        assert_eq!(lexer.next().unwrap(), Token::Number(0));
        assert_eq!(lexer.next().unwrap(), Token::Arguments);
        assert_eq!(lexer.next().unwrap(), Token::LeftCurly);
        assert_eq!(lexer.next().unwrap(), Token::Type(Type::INT));
    }
}
