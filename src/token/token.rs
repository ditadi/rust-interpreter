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
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Token {
        let token = Token {
            token_type,
            literal: ch.to_string(),
        };
        return token;
    }
}
