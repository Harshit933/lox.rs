// Tokens are lexemes only with a bit of more information
use crate::{literal::LiteralValue, token_type::TokenType};

// This will be the tokens
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // FIXME: We should use Option here
    pub literal: LiteralValue,
    pub line: u16,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: u16) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "token_type: {}, lexeme: {}, literal: {}, line: {}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}
