#[allow(unused_variables)]

use crate::{error::{parser_error, LoxErrors}, literal::LiteralValue};
use crate::{expr::Expr, token::Token, token_type::TokenType};

/**
*  Parser grammer
*  expression     → equality ;
*  equality       → comparison ( ( "!=" | "==" ) comparison )* ;
*  comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*  term           → factor ( ( "-" | "+" ) factor )* ;
*  factor         → unary ( ( "/" | "*" ) unary )* ;
*  unary          → ( "!" | "-" ) unary
*                   | primary ;
*  primary        → NUMBER | STRING | "true" | "false" | "nil"
                    | "(" expression ")" ;
*/

/**
 * Associativity table:
 *
 * Name	        Operators	Associates
 * Equality	    == !=	    Left
 * Comparison	> >= < <=	Left
 * Term	-       +	        Left
 * Factor	    / *	        Left
 * Unary	    ! -	        Right
 */

/**
 * Precedence table:
 * expression     → ...
 * equality       → ...
 * comparison     → ...
 * term           → ...
 * factor         → ...
 * unary          → ...
 * primary        → ...

The precedence if from lower to higher, meaning the primary has the highest precedence
*/

// Main struct used for parsing stuff!
pub struct Parser {
    tokens_list: Vec<Token>,
    // The token where we are at now!
    current: u16,
}

impl Parser {
    pub fn new(tokens_list: Vec<Token>) -> Self {
        Self {
            tokens_list,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    // An expression expr
    fn expression(&mut self) -> Result<Expr, LoxErrors> {
        return Ok(self.equality());
    }

    // An equality expr
    fn equality(&mut self) -> Expr {
        // Comparison
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            // Operator
            let operator = self.previous();
            // Comparsion
            let expr_temp = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(expr_temp),
            };
        }

        return expr;
    }

    // A comparison expr
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let temp_term = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(temp_term),
            };
        }
        return expr;
    }

    // A term expr
    fn term(&mut self) -> Expr {
        let mut expr_factor = self.factor();

        while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let factor = self.factor();
            expr_factor = Expr::Binary {
                left: Box::new(expr_factor),
                operator,
                right: Box::new(factor),
            };
        }

        return expr_factor;
    }

    // A factor expr
    fn factor(&mut self) -> Expr {
        let mut expr_unary = self.unary();

        while self.match_tokens(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let unary = self.unary();

            expr_unary = Expr::Binary {
                left: Box::new(expr_unary),
                operator,
                right: Box::new(unary),
            };
        }

        return expr_unary;
    }

    // A unary expr
    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let unary = self.unary();

            return Expr::Unary {
                operator,
                right: Box::new(unary),
            };
        }

        // TODO: Handle the error here
        return self.primary().unwrap();
    }

    // A primary expr
    fn primary(&mut self) -> Result<Expr, LoxErrors> {

        if self.match_tokens(&[TokenType::FALSE]) {
            return Ok(Expr::Literal { value: LiteralValue::Boolean(false) })
        }

        if self.match_tokens(&[TokenType::TRUE]) {
            return Ok(Expr::Literal { value: LiteralValue::Boolean(true) })
        }

        if self.match_tokens(&[TokenType::NIL]) {
            return Ok(Expr::Literal { value: LiteralValue::Null })
        }

        if self.match_tokens(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal { value: self.previous().literal })
        }

        if self.match_tokens(&[TokenType::LEFT_PAREN]) {
            let expr= self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        Err(self.error(self.peek(), "Expect expression."))

        // let expr = match &self.peek().token_type {
        //     TokenType::FALSE => Expr::Literal {
        //         value: LiteralValue::Boolean(false),
        //     },
        //     TokenType::TRUE => Expr::Literal {
        //         value: LiteralValue::Boolean(true),
        //     },
        //     TokenType::NIL => Expr::Literal {
        //         value: LiteralValue::Null,
        //     },
        //     TokenType::STRING => Expr::Literal {
        //         value: self.peek().literal.clone(),
        //     },
        //     TokenType::NUMBER => Expr::Literal {
        //         value: self.peek().literal.clone(),
        //     },
        //     TokenType::LEFT_PAREN => {
        //         let expr = self.expression()?;
        //         self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.")?;
        //         Expr::Grouping {
        //             expression: Box::new(expr),
        //         }
        //     }
        //     _ => {
        //         println!("WE FOUND THISSS: {:?}", self.peek());
        //         return Err(self.error(self.peek(), "Expect expression."))
        //     },
        // };

        // self.advance();

        // Ok(expr)
    }

    // Synchronize
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => self.advance(),
            };
        }
    }

    // Wrapper around parser error
    fn error(&self, token: &Token, message: &str) -> LoxErrors {
        parser_error(token, message);
        LoxErrors::PARSEERROR(token.clone())
    }

    // Consume the current token
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxErrors> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(LoxErrors::UNEXPECTEDTOKENTYPEFOUND(token_type))
    }

    // Match all the tokens that we need our current token to be!
    fn match_tokens(&mut self, array_of_tokens: &[TokenType]) -> bool {
        for token in array_of_tokens {
            if self.check(&token) {
                // Consume the token
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// TODO(SAFETY): Check if unwrap() here is safe or not?
    fn previous(&self) -> Token {
        self.tokens_list
            .get((self.current - 1) as usize)
            .unwrap()
            .clone()
    }

    /// Check the current token if it matches with the TokenType we are
    /// looking for
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        // Check the current token is of the same type that we needed
        println!("INSIDE CHECK: PEEK: {:?} and token_type that we are checking: {:?}", &self.peek().token_type, token_type);
        // println!("INSIDE CHECK ACTUAL");
        return self.peek().token_type.eq(token_type);
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            // Consuming the current token
            self.current = self.current + 1;
        }

        // Returning the previous token (Current token has been updated!)
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.tokens_list
            .get(self.current as usize)
            .unwrap()
            .token_type
            == TokenType::EOF
    }

    // Peek the current token
    pub fn peek(&self) -> &Token {
        // println!("PEEK FUNCTION!!! : {:?}", self.current as usize);
        self.tokens_list.get(self.current as usize).unwrap()
    }
}

mod tests {
    use crate::{
        expr::{self, AstPrinter}, scanner::Scanner, token::Token
    };

    use super::Parser;

    use crate::token_type::TokenType::{EOF, EQUAL_EQUAL, IDENTIFIER};
    use crate::literal::LiteralValue;

    #[test]
    fn test_equality() {
         let mut scanner = Scanner::new("-123 * 45.67".to_string());
         let tokens = scanner.scan_tokens().unwrap();
         println!("THESE ARE THE TOKENS: {:?}", tokens);
 
         let mut parser = Parser::new(tokens);
         let expression = parser.parse().expect("Could not parse sample code.");
         let printer = AstPrinter;
 
         assert_eq!(printer.print(expression), "(* (- 123) 45.67)");
    }
}
