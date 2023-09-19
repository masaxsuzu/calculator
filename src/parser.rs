use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
        };

        parser.advance_token();
        parser.advance_token();

        return parser;
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program: Program = vec![];

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => {
                    program.push(statement);
                }
                Err(e) => return Err(e),
            }
            self.advance_token();
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token {
            Token::Illegal => Err(ParseError::FoundIllegalToken),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        match self.parse_expression(Precedence::Lowest) {
            Ok(expr) => Ok(Statement::Expression(expr)),
            Err(e) => Err(e),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        let mut left = match self.current_token {
            Token::Integer(_) => self.parse_int_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::LeftParen => self.parse_grouped_expression(),
            _ => {
                return Err(ParseError::FoundUnexpectedToken);
            }
        };

        while precedence < self.next_precedence() {
            match self.next_token {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                    self.advance_token();
                    left = self.parse_infix_expression(left.unwrap());
                }
                _ => return left,
            }
        }
        return left;
    }

    fn parse_prefix_expression(&mut self) -> Result<Expr, ParseError> {
        let prefix = match self.current_token {
            Token::Minus => Prefix::Minus,
            _ => unreachable!("parse_prefix_expression"),
        };

        self.advance_token();

        match self.parse_expression(Precedence::Prefix) {
            Ok(expression) => Ok(Expr::Prefix(prefix, Box::new(expression))),
            Err(e) => Err(e),
        }
    }

    fn parse_infix_expression(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let infix = match self.current_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Multiply,
            Token::Slash => Infix::Divide,
            _ => unreachable!("parse_infix_expression"),
        };

        let precedence = self.current_precedence();

        self.advance_token();

        match self.parse_expression(precedence) {
            Ok(expression) => Ok(Expr::Infix(infix, Box::new(left), Box::new(expression))),
            Err(e) => Err(e),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expr, ParseError> {
        self.advance_token();

        let expression = self.parse_expression(Precedence::Lowest);

        match self.expect_next_token(Token::RightParen) {
            true => expression,
            _ => Err(ParseError::FoundUnterminatedParentheses),
        }
    }

    fn parse_int_expression(&mut self) -> Result<Expr, ParseError> {
        match self.current_token {
            Token::Integer(ref mut int) => Ok(Expr::Literal(Literal::Int(int.clone()))),
            _ => unreachable!("parse_int_expression"),
        }
    }

    fn advance_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.next_token == token.clone() {
            self.advance_token();
            return true;
        } else {
            return false;
        }
    }

    fn current_precedence(&mut self) -> Precedence {
        return Self::token_to_precedence(&self.current_token);
    }

    fn next_precedence(&mut self) -> Precedence {
        return Self::token_to_precedence(&self.next_token);
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::error::ParseError;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    #[test]
    fn test_literals() {
        let tests = vec![
            (
                r#"1000
                "#,
                vec![Statement::Expression(Expr::Literal(Literal::Int(1000)))],
            ),
            (
                r#"-2000
                "#,
                vec![Statement::Expression(Expr::Prefix(
                    Prefix::Minus,
                    Box::new(Expr::Literal(Literal::Int(2000))),
                ))],
            ),
            (
                r#"3000 + 4000
                "#,
                vec![Statement::Expression(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(3000))),
                    Box::new(Expr::Literal(Literal::Int(4000))),
                ))],
            ),
            (
                r#"3000 - 4000
                "#,
                vec![Statement::Expression(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Literal(Literal::Int(3000))),
                    Box::new(Expr::Literal(Literal::Int(4000))),
                ))],
            ),
            (
                r#"3000 * 4000
                "#,
                vec![Statement::Expression(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Literal(Literal::Int(3000))),
                    Box::new(Expr::Literal(Literal::Int(4000))),
                ))],
            ),
            (
                r#"3000 / 4000
                "#,
                vec![Statement::Expression(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Literal(Literal::Int(3000))),
                    Box::new(Expr::Literal(Literal::Int(4000))),
                ))],
            ),
            (
                r#"(3000 + 4000 ) / 10
                "#,
                vec![Statement::Expression(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(3000))),
                        Box::new(Expr::Literal(Literal::Int(4000))),
                    )),
                    Box::new(Expr::Literal(Literal::Int(10))),
                ))],
            ),
        ];

        for (input, want) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse().unwrap();
            assert_program(want, program);
        }
    }
    fn assert_program(want: Vec<Statement>, got: Program) {
        assert_eq!(want, got)
    }

    #[test]
    fn test_found_illegal_token() {
        let tests = vec![
            (
                r#"1x
                "#,
                ParseError::FoundIllegalToken,
            ),
            (
                r#")
                "#,
                ParseError::FoundUnexpectedToken,
            ),
            (
                r#"(1
                "#,
                ParseError::FoundUnterminatedParentheses,
            ),
        ];

        for (input, want) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let error = parser.parse().unwrap_err();
            assert_eq!(want, error);
        }
    }
}
