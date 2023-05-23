// primeagen test help

#[cfg(test)]
mod test {

    use crate::token::token::TokenType;
    use crate::{lexer::lexer::Lexer, token::token::Token};

    #[test]
    fn get_next_token() -> Result<(), String> {
        let input = "=-/+*(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::new(TokenType::EQUAL, 0),
            Token::new(TokenType::MINUS, 0),
            Token::new(TokenType::DIVIDER, 0),
            Token::new(TokenType::PLUS, 0),
            Token::new(TokenType::MULTIPLIER, 0),
            Token::new(TokenType::LPAREN, 0),
            Token::new(TokenType::RPAREN, 0),
            Token::new(TokenType::LBRACE, 0),
            Token::new(TokenType::RBRACE, 0),
            Token::new(TokenType::COMMA, 0),
            Token::new(TokenType::SEMICOLON, 0),
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
