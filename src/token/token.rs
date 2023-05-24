#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT(String),
    INTEGER(String),
    EQUAL,
    PLUS,
    MINUS,
    DIVIDER,
    MULTIPLIER,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        let token = Token {
            token_type,
            literal: ch,
        };
        return token;
    }

    pub fn get_identifier_token(ident: String) -> Result<TokenType, String> {
        match ident.as_str() {
            "fn" => Ok(TokenType::FUNCTION),
            "let" => Ok(TokenType::LET),
            "true" => Ok(TokenType::TRUE),
            "false" => Ok(TokenType::FALSE),
            "if" => Ok(TokenType::IF),
            "else" => Ok(TokenType::ELSE),
            "return" => Ok(TokenType::RETURN),
            _ => Ok(TokenType::IDENT(ident)),
        }
    }
}
