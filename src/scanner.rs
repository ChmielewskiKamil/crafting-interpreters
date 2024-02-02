use crate::token::{Location, Token, TokenType};

pub struct Scanner {
    source_file: Vec<u8>,
    scanned_tokens: Vec<Token>,
    current_char: usize,
    current_line: usize,
    current_lexeme_start: usize,
}

impl Scanner {
    fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.current_lexeme_start = self.current_char;
            self.scan_token();
        }

        self.scanned_tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            location: Location {
                offset: self.current_lexeme_start,
                length: 0,
            },
        });
        self.scanned_tokens
    }

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        self.current_char >= self.source_file.len()
    }
}
