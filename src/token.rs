#[derive(Debug)]
enum TokenType {
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
struct Location {
    // Offset from the beginning of the file to the beginning of the lexeme
    offset: u32,
    // Length of the lexeme
    length: u32,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    location: Location,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "Token: {:?}, Lexeme: {}, Location: {:?}",
            self.token_type, self.lexeme, self.location
        )
    }
}
