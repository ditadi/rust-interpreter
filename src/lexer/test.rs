// primeagen test help

#[cfg(test)]
mod test {

    use crate::token::token::TokenType;
    use crate::{lexer::lexer::Lexer, token::token::Token};

    #[test]
    fn get_next_token() -> Result<(), String> {
        let input = "=-/+*(){},;";
        let mut lexer = Lexer::new(input.chars().collect());

        let tokens = vec![
            Token::new(TokenType::EQUAL, '0'),
            Token::new(TokenType::MINUS, '0'),
            Token::new(TokenType::DIVIDER, '0'),
            Token::new(TokenType::PLUS, '0'),
            Token::new(TokenType::MULTIPLIER, '0'),
            Token::new(TokenType::LPAREN, '0'),
            Token::new(TokenType::RPAREN, '0'),
            Token::new(TokenType::LBRACE, '0'),
            Token::new(TokenType::RBRACE, '0'),
            Token::new(TokenType::COMMA, '0'),
            Token::new(TokenType::SEMICOLON, '0'),
        ];
        for token in tokens {
            let next_token = lexer.next_token()?;
            println!(
                "expected: {:?}, received {:?}",
                token.token_type, next_token.token_type
            );
            assert_eq!(token.token_type, next_token.token_type);
        }

        return Ok(());
    }
    #[test]
    fn test_monkey_code() -> Result<(), String> {
        let input = "
let five = 5;

let ten = 10; 

let add = fn(x, y) {
  x + y;
}

let result = add(five, ten);
";

        let mut lexer = Lexer::new(input.chars().collect());

        let tokens = vec![
            Token::new(TokenType::LET, '0'),
            Token::new(TokenType::IDENT(String::from("five")), '0'),
            Token::new(TokenType::EQUAL, '0'),
            Token::new(TokenType::INTEGER(String::from("5")), '0'),
            Token::new(TokenType::SEMICOLON, '0'),
            Token::new(TokenType::LET, '0'),
            Token::new(TokenType::IDENT(String::from("ten")), '0'),
            Token::new(TokenType::EQUAL, '0'),
            Token::new(TokenType::INTEGER(String::from("10")), '0'),
            Token::new(TokenType::SEMICOLON, '0'),
            Token::new(TokenType::LET, '0'),
            Token::new(TokenType::IDENT(String::from("add")), '0'),
            Token::new(TokenType::EQUAL, '0'),
            Token::new(TokenType::FUNCTION, '0'),
            Token::new(TokenType::LPAREN, '0'),
            Token::new(TokenType::IDENT(String::from("x")), '0'),
            Token::new(TokenType::COMMA, '0'),
            Token::new(TokenType::IDENT(String::from("y")), '0'),
            Token::new(TokenType::RPAREN, '0'),
            Token::new(TokenType::LBRACE, '0'),
            Token::new(TokenType::IDENT(String::from("x")), '0'),
            Token::new(TokenType::PLUS, '0'),
            Token::new(TokenType::IDENT(String::from("y")), '0'),
            Token::new(TokenType::SEMICOLON, '0'),
            Token::new(TokenType::RBRACE, '0'),
            Token::new(TokenType::LET, '0'),
            Token::new(TokenType::IDENT(String::from("result")), '0'),
            Token::new(TokenType::EQUAL, '0'),
            Token::new(TokenType::IDENT(String::from("add")), '0'),
            Token::new(TokenType::LPAREN, '0'),
            Token::new(TokenType::IDENT(String::from("five")), '0'),
            Token::new(TokenType::COMMA, '0'),
            Token::new(TokenType::IDENT(String::from("ten")), '0'),
            Token::new(TokenType::RPAREN, '0'),
            Token::new(TokenType::SEMICOLON, '0'),
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!(
                "expected: {:?}, received {:?}",
                token.token_type, next_token.token_type
            );
            assert_eq!(token.token_type, next_token.token_type);
        }

        return Ok(());
    }
}
