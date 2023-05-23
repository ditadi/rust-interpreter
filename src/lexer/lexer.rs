pub use crate::token::token::Token;
use crate::token::token::TokenType;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        let token = match self.ch {
            b'=' => Token::new(TokenType::EQUAL, self.ch),
            b'+' => Token::new(TokenType::PLUS, self.ch),
            b'-' => Token::new(TokenType::MINUS, self.ch),
            b'/' => Token::new(TokenType::DIVIDER, self.ch),
            b'*' => Token::new(TokenType::MULTIPLIER, self.ch),
            b',' => Token::new(TokenType::COMMA, self.ch),
            b';' => Token::new(TokenType::SEMICOLON, self.ch),
            b'(' => Token::new(TokenType::LPAREN, self.ch),
            b')' => Token::new(TokenType::RPAREN, self.ch),
            b'{' => Token::new(TokenType::LBRACE, self.ch),
            b'}' => Token::new(TokenType::RBRACE, self.ch),
            _ => todo!("..."),
        };

        self.read_char();

        return Ok(token);
    }
}
