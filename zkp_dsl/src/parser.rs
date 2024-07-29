use crate::lexer::{Lexer, Token};
use crate::ast::{Program, Statement, Expr, BinaryOperator};

/**
Expr：表示表达式，可以是标识符、数字或二元操作。
BinaryOperator：表示二元操作符。
Statement：表示语句，包括输入声明、约束定义和断言。
Program：表示整个程序，由一系列语句组成。
 **/
pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        let statement =  match self.current_token {
            Token::Input => self.parse_input(),
            Token::Constraint => self.parse_constraint(),
            Token::Assert => self.parse_assert(),
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        };
        self.expect_semicolon()?;

        statement
    }

    fn parse_input(&mut self) -> Result<Statement, String> {
        self.next_token();
        let is_private = if self.current_token == Token::Private {
            self.next_token();
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;
        self.expect_token(Token::Colon)?;
        let typ = self.parse_identifier()?;
        Ok(Statement::Input {
            is_private,
            name,
            typ,
        })
    }

    fn parse_constraint(&mut self) -> Result<Statement, String> {
        self.next_token();
        let name = self.parse_identifier()?;
        self.expect_token(Token::LeftBrace)?;
        let expr = self.parse_expression(0)?;
        self.expect_token(Token::RightBrace)?;

        Ok(Statement::Constraint { name, expr })
    }

    fn parse_assert(&mut self) -> Result<Statement, String> {
        self.next_token();
        let expr = self.parse_expression(0)?;
        Ok(Statement::Assert(expr))
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while let Some(op) = self.get_binary_operator() {
            let op_precedence = self.get_precedence(&op);
            if op_precedence < precedence {
                break;
            }
            self.next_token();
            let right = self.parse_expression(op_precedence)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Token::Identifier(name) => {
                let expr = Expr::Identifier(name.clone());
                self.next_token();
                Ok(expr)
            }
            Token::Number(value) => {
                let expr = Expr::Number(*value);
                self.next_token();
                Ok(expr)
            }
            Token::LeftParen => {
                self.next_token();
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn get_binary_operator(&self) -> Option<BinaryOperator> {
        match self.current_token {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Subtract),
            Token::Star => Some(BinaryOperator::Multiply),
            Token::Slash => Some(BinaryOperator::Divide),
            Token::DoubleEquals => Some(BinaryOperator::DoubleEquals),
            Token::NotEquals => Some(BinaryOperator::NotEquals),
            Token::LessThan => Some(BinaryOperator::LessThan),
            Token::GreaterThan => Some(BinaryOperator::GreaterThan),
            Token::LessThanEquals => Some(BinaryOperator::LessThanEquals),
            Token::GreaterThanEquals => Some(BinaryOperator::GreaterThanEquals),
            _ => None,
        }
    }

    fn get_precedence(&self, op: &BinaryOperator) -> u8 {
        match op {
            BinaryOperator::Add | BinaryOperator::Subtract => 1,
            BinaryOperator::Multiply | BinaryOperator::Divide => 2,
            BinaryOperator::DoubleEquals | BinaryOperator::NotEquals |
            BinaryOperator::LessThan | BinaryOperator::GreaterThan |
            BinaryOperator::LessThanEquals | BinaryOperator::GreaterThanEquals => 0,
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            Ok(name)
        } else {
            Err(format!("Expected identifier, got {:?}", self.current_token))
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.next_token();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token))
        }
    }
    fn expect_semicolon(&mut self) -> Result<(), String> {
        if self.current_token == Token::Semicolon {
            Ok(())
        } else {
            Err("Expected semicolon".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Program, Statement, Expr, BinaryOperator};

    fn parse(input: &str) -> Result<Program, String> {
        let mut parser = Parser::new(input);
        parser.parse_program()
    }

    #[test]
    fn test_parse_input_declaration() {
        let input = "input private x: Field;";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Input { is_private, name, typ } => {
                assert!(is_private);
                assert_eq!(name, "x");
                assert_eq!(typ, "Field");
            }
            _ => panic!("Expected Input statement"),
        }
    }

    #[test]
    fn test_parse_constraint() {
        let input = "constraint c1 { x * y == 10 };";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Constraint { name, expr } => {
                assert_eq!(name, "c1");
                match expr {
                    Expr::BinaryOp { left, operator, right } => {
                        assert_eq!(
                            **left,
                            Expr::BinaryOp {
                                left: Box::new(Expr::Identifier("x".to_string())),
                                operator: BinaryOperator::Multiply,
                                right: Box::new(Expr::Identifier("y".to_string())),
                            }
                        );
                        assert_eq!(*operator, BinaryOperator::DoubleEquals);
                        assert_eq!(**right, Expr::Number(10));
                    }
                    _ => panic!("Expected BinaryOp expression"),
                }
            }
            _ => panic!("Expected Constraint statement"),
        }
    }
}
