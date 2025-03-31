// Make something from here https://craftinginterpreters.com/scanning.html#error-handling

use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub enum LoxErrors {
    INVALIDCHARCTER(String),
    UNTERMINATEDSTRING(),
    CANNOTFINDSUBSTRING(String),
    UNEXPECTEDTOKENTYPEFOUND(TokenType),
    PRIMARYEXPRERROR(Token),
    PARSEERROR(Token),
}

impl std::fmt::Display for LoxErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxErrors::INVALIDCHARCTER(char) => write!(f, "Invalid character found while parsing: {}", char),
            LoxErrors::UNTERMINATEDSTRING() => write!(f, "String not terminated"),
            LoxErrors::CANNOTFINDSUBSTRING(string) => write!(f, "Cannot find the specified substring from the string: {}", string),
            LoxErrors::UNEXPECTEDTOKENTYPEFOUND(token_type) => write!(f, "Unexpected token found: {:?}", token_type),
            LoxErrors::PRIMARYEXPRERROR(token) => write!(f, "Unexpected token found while parsing primary expr: {:?}", token),
            LoxErrors::PARSEERROR(token) => write!(f, "Error while parsing : {:?}", token),
        }
    }
}


pub fn report(line: i32, where_: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, where_, message);
}

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

/// A parser error 
pub fn parser_error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.line as i32, " at end", message);
    } else {
        report(token.line as i32, &format!(" at '{}'", token.lexeme), message);
    }
}