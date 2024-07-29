use std::iter::Peekable;
use std::str::Chars;
#[derive(Debug,PartialEq,Clone)]
pub(crate) enum Token{
    //关键字
    Input,
    Private,
    Constraint,
    Assert,
    For,
    Function,

    // 标识符和字面量
    Identifier(String),
    Number(i64),

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    DoubleEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,

    // 其他符号
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Colon,
    Semicolon,

    // 文件结束标记
    EOF,
}

pub(crate) struct Lexer<'a>{
    input: Peekable<Chars<'a>>,
}
impl<'a> Lexer<'a>{
    pub(crate) fn new(input: &'a str) ->Self{
        Lexer{
            input:input.chars().peekable(),
        }
    }
    pub(crate) fn next_token(&mut self) ->Token{
        self.skip_whitespace();

        match self.input.next() {
        Some(ch) => match ch {
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(ch),
            '0'..='9' => self.lex_number(ch),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '!' => self.lex_not_equals(),
            '=' => self.lex_equals(),
            '<' => self.lex_less_than(),
            '>' => self.lex_greater_than(),
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            _ => panic!("Unexpected character: {}", ch),
        },
        None => Token::EOF,
    }
    }
    fn skip_whitespace(&mut self){
        while let Some(&ch) = self.input.peek(){
            if !ch.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    fn lex_identifier(&mut self,first_char:char) ->Token{
        let mut identifier = String::new();
        identifier.push(first_char);
        while let Some (&ch) = self.input.peek(){
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(self.input.next().unwrap());
            }else {
                break;
            }
        }
        match identifier.as_str() {
            "input" => Token::Input,
            "private" => Token::Private,
            "constraint" => Token::Constraint,
            "assert" => Token::Assert,
            "for" => Token::For,
            "function" => Token::Function,
            _ => Token::Identifier(identifier),
        }
    }
    fn lex_number(&mut self, first_digit: char) -> Token {
        let mut number = String::new();
        number.push(first_digit);

        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                number.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap())
    }
    fn lex_equals(&mut self) -> Token {
        if let Some(&'=') = self.input.peek() {
            self.input.next();
            Token::DoubleEquals
        } else {
            Token::Equals
        }
    }
    fn lex_not_equals(&mut self) -> Token {
        if let Some(&'=') = self.input.peek() {
            self.input.next();
            Token::NotEquals
        } else {
            // 如果只有一个 '!', 这里可能需要返回一个不同的 Token 或报错
            panic!("Expected '=' after '!'")
        }
    }
    fn lex_less_than(&mut self) -> Token {
        if let Some(&'=') = self.input.peek() {
            self.input.next();
            Token::LessThanEquals
        } else {
            Token::LessThan
        }
    }
    fn lex_greater_than(&mut self) -> Token {
        if let Some(&'=') = self.input.peek() {
            self.input.next();
            Token::GreaterThanEquals
        } else {
            Token::GreaterThan
        }
    }
}
// 使用示例
// fn main() {
//     let input = "input private x: Field";
//     let mut lexer = Lexer::new(input);
//
//     loop {
//         let token = lexer.next_token();
//         println!("{:?}", token);
//         if token == Token::EOF {
//             break;
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: &str, expected_tokens: Vec<Token>) {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_keywords() {
        test_lexer(
            "input private constraint assert for function",
            vec![
                Token::Input,
                Token::Private,
                Token::Constraint,
                Token::Assert,
                Token::For,
                Token::Function,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        test_lexer(
            "x y123 _test camelCase",
            vec![
                Token::Identifier("x".to_string()),
                Token::Identifier("y123".to_string()),
                Token::Identifier("_test".to_string()),
                Token::Identifier("camelCase".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_numbers() {
        test_lexer(
            "0 123 9876543210",
            vec![
                Token::Number(0),
                Token::Number(123),
                Token::Number(9876543210),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_operators() {
        test_lexer(
            "+ - * / = == != < > <= >=",
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Equals,
                Token::DoubleEquals,
                Token::NotEquals,
                Token::LessThan,
                Token::GreaterThan,
                Token::LessThanEquals,
                Token::GreaterThanEquals,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_punctuation() {
        test_lexer(
            "{ } ( ) : ;",
            vec![
                Token::LeftBrace,
                Token::RightBrace,
                Token::LeftParen,
                Token::RightParen,
                Token::Colon,
                Token::Semicolon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_complex_input() {
        test_lexer(
            "input private x: Field\nconstraint c1 { x * y == 10 }",
            vec![
                Token::Input,
                Token::Private,
                Token::Identifier("x".to_string()),
                Token::Colon,
                Token::Identifier("Field".to_string()),
                Token::Constraint,
                Token::Identifier("c1".to_string()),
                Token::LeftBrace,
                Token::Identifier("x".to_string()),
                Token::Star,
                Token::Identifier("y".to_string()),
                Token::DoubleEquals,
                Token::Number(10),
                Token::RightBrace,
                Token::EOF
            ]
        );
    }
}