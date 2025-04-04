use crate::{literal::LiteralValue, token::Token};

/**
* expression     → literal
                   | unary
                   | binary
                   | grouping ;

   literal        → NUMBER | STRING | "true" | "false" | "nil" ;
   grouping       → "(" expression ")" ;
   unary          → ( "-" | "!" ) expression ;
   binary         → expression operator expression ;
   operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
                    | "+"  | "-"  | "*" | "/" ;
*/

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

// Visitor pattern implementation
pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&self, expression: &Expr) -> R;
    fn visit_literal_expr(&self, value: String) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary {left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value.to_string()),
            Expr::Unary {operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            r.push_str(&e.accept(self));
        }
        r.push_str(")");
        r
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal_expr(&self, value: String) -> String {
        value // check for null
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{token::Token, token_type::TokenType};
    
    #[test]
    fn test_printer() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::MINUS,"-".to_string(), LiteralValue::Null, 1),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(123.),
                }),
            }),
            operator: Token::new(TokenType::STAR, "*".to_string(), LiteralValue::Null, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: LiteralValue::Number(45.67),
                }),
            }),
        };
        let printer = AstPrinter;
        // println!("This is the {}", printer.print(expression));

        assert_eq!(printer.print(expression), "(* (- 123) (group 45.67))");
    }
}