#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    Plus,
    Minus,
    Slash,
    Star,

    // One or two character tokens
    Equal,

    // Literals
    Number,
    String,
    Identifier,

    // Keywords
    Function,
    True,
    False,

    EOF,
}

#[derive(Debug)]
pub struct Location {
    // Offset from the beginning of the file to the beginning of the lexeme
    pub offset: usize,
    // Length of the lexeme
    pub length: usize,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: Location,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "Token: {:?}, Lexeme: {}, Location: {:?}",
            self.token_type, self.lexeme, self.location
        )
    }
}
