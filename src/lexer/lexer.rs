pub use crate::token::token::Token;
use crate::token::token::TokenType;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.position < self.input.len() && self.ch.is_ascii_alphanumeric() {
            self.read_char();
        }
        return self.input[position..self.position].iter().collect();
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.position < self.input.len() && self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        return self.input[position..self.position].iter().collect();
    }

    pub fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespaces();

        let tok = match self.ch {
            '=' => Token::new(TokenType::EQUAL, self.ch),
            '+' => Token::new(TokenType::PLUS, self.ch),
            '-' => Token::new(TokenType::MINUS, self.ch),
            '/' => Token::new(TokenType::DIVIDER, self.ch),
            '*' => Token::new(TokenType::MULTIPLIER, self.ch),
            ',' => Token::new(TokenType::COMMA, self.ch),
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let identifier: String = self.read_identifier();
                    match Token::get_identifier_token(identifier) {
                        Ok(keyword_token) => {
                            return Ok(Token::new(keyword_token, self.ch));
                        }
                        Err(_err) => {
                            return Ok(Token::new(
                                TokenType::IDENT(self.read_identifier()),
                                self.ch,
                            ));
                        }
                    }
                } else if self.ch.is_ascii_alphanumeric() {
                    let identifier: String = self.read_number();
                    return Ok(Token::new(TokenType::INTEGER(identifier), self.ch));
                } else {
                    return Ok(Token::new(TokenType::ILLEGAL, self.ch));
                }
            }
        };

        self.read_char();

        return Ok(tok);
    }
}
