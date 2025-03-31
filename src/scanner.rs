use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::literal::LiteralValue;
use crate::token::Token;
use crate::{error::LoxErrors, token_type::TokenType};

// This would be the the Scanner object
pub struct Scanner {
    // All the characters in the file
    pub source: String,
    // Beginning of the current lexeme
    start: u16,
    // Character we are at currently of the lexeme
    current: u16,
    line: u16,
    tokens: Vec<Token>,
}

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);
        keywords
    };
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxErrors> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            LiteralValue::Null,
            self.line,
        ));
        Ok(self.tokens.clone())
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u16;
    }

    pub fn scan_token(&mut self) -> Result<(), LoxErrors> {
        let character = self.advance();
        match character {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                // println!("Current char: {}, {}", character, self.current);
                if self.match_next('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line = self.line + 1;
            }
            '"' => self.string()?,
            _ => {
                if self.is_digit(character) {
                    // CASE FOR NUMBERS
                    return self.number();
                } else if self.is_alpha(character) {
                    // CASE FOR SPECIAL CHARACTERS
                    return self.identifier();
                }
                return Err(LoxErrors::INVALIDCHARCTER(character.to_string()));
            }
        }

        Ok(())
    }

    pub fn identifier(&mut self) -> Result<(), LoxErrors> {
        // Scan till we are getting alphanumeric characters
        while self.is_alphanumic(self.peek()) {
            self.advance();
        }

        // See if the current value is some type of special character and if it
        // is then add that specific token and if it is not then it is an identifier
        // Just add it.
        let value = self.substring(&self.source, self.start.into(), self.current.into())?;
        let special_type = HASHMAP.get(value.as_str());

        if let Some(val) = special_type {
            self.add_token(val.clone());
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }

        Ok(())
    }

    fn is_digit(&self, character: char) -> bool {
        let number_array = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if number_array.contains(&character) {
            return true;
        }

        return false;
    }

    fn is_alpha(&self, character: char) -> bool {
        if (character >= 'A' && character <= 'Z')
            || (character >= 'a' && character <= 'z')
            || character == '_'
        {
            return true;
        }

        false
    }

    fn is_alphanumic(&self, character: char) -> bool {
        self.is_alpha(character) || self.is_digit(character)
    }

    // Function to process numbers
    fn number(&mut self) -> Result<(), LoxErrors> {
        // Peek and see if the next char in the lexeme is a digit or not
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // For decimals
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        // Add the token
        self.add_token_number(
            TokenType::NUMBER,
            self.substring(&self.source, self.start.into(), self.current.into())?,
        )?;

        Ok(())
    }

    // Peek the current character
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        // SAFETY: Safe to unwrap
        self.source.chars().nth(self.current as usize).unwrap()
    }

    // Peek the next character
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u16 {
            return '\0';
        }
        // SAFETY: Safe to unwrap
        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }

    pub fn match_next(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        // SAFETY: Safe to unwrap() we are not at the end
        println!(
            "inside next match: {}",
            &self.source.chars().nth((self.current).into()).unwrap()
        );
        if &self.source.chars().nth((self.current).into()).unwrap() == &expected_char {
            self.current = self.current + 1;
            return true;
        }

        false
    }

    /// Used for advancing character in a lexeme
    /// We consume the current char and return it, then shift to the next char
    pub fn advance(&mut self) -> char {
        // SAFETY: Safe to unwrap here as we check for EOF inside is_at_end()
        let res = self.source.chars().nth((self.current).into()).unwrap();
        self.current = self.current + 1;
        res
    }

    pub fn string(&mut self) -> Result<(), LoxErrors> {
        // Ending the string val
        while self.peek() != '"' && !self.is_at_end() {
            // For new line, just modify our current line to be line + 1
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        // There could be a case where the left '"' is not specified -
        if self.is_at_end() {
            return Err(LoxErrors::UNTERMINATEDSTRING());
        }

        // Advancing if everything goes well!
        self.advance();

        // Forming the string
        let value = self
            .substring(
                &self.source,
                (self.start + 1).into(),
                (self.current - 1).into(),
            )
            .unwrap();
        self.add_token_string(TokenType::STRING, value)?;

        Ok(())
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_priv(token_type, "nil".to_string());
    }

    pub fn add_token_string(
        &mut self,
        token_type: TokenType,
        literal: String,
    ) -> Result<(), LoxErrors> {
        // TODO: Find a way for this
        let text = self
            .substring(&self.source, self.start.into(), self.current.into())
            .unwrap();

        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            LiteralValue::String(literal),
            self.line,
        ));

        Ok(())
    }

    // Add a token for number
    pub fn add_token_number(
        &mut self,
        token_type: TokenType,
        literal: String,
    ) -> Result<(), LoxErrors> {
        // TODO: Find a way for this
        let text = self
            .substring(&self.source, self.start.into(), self.current.into())
            .unwrap().parse::<f64>().unwrap();

        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            LiteralValue::Number(text),
            self.line,
        ));

        Ok(())
    }

    // TODO: Move this to another struct (LoxCommon)
    // Function to find substring
    // TODO: The arguments to these functions could be generic
    pub fn substring(
        &self,
        string_to_work: &str,
        start_index: usize,
        end_index: usize,
    ) -> Result<String, LoxErrors> {
        Ok(string_to_work[(start_index) as usize..(end_index) as usize].to_string())
    }

    pub fn add_token_priv(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source.as_str()[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), LiteralValue::String(literal), self.line));
    }
}
