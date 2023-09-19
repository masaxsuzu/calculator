use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token = match self.ch {
            b'0'..=b'9' => return self.consume_number(),

            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,

            b'(' => Token::LeftParen,
            b')' => Token::RightParen,

            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        return token;
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        while let b'0'..=b'9' = self.ch {
            self.read_char();
        }

        let consumed = &self.input[start_pos..self.pos];

        return Token::Integer(consumed.parse::<i64>().unwrap());
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
            return;
        }

        self.ch = self.input.as_bytes()[self.next_pos];
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => self.read_char(),
                _ => break,
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"
        1 + 2
        3 - 4
        5 * 6
        7 / 8
        ()
        9*10

        &
        "#;
        let tests = vec![
            Token::Integer(1),
            Token::Plus,
            Token::Integer(2),
            Token::Integer(3),
            Token::Minus,
            Token::Integer(4),
            Token::Integer(5),
            Token::Asterisk,
            Token::Integer(6),
            Token::Integer(7),
            Token::Slash,
            Token::Integer(8),
            Token::LeftParen,
            Token::RightParen,
            Token::Integer(9),
            Token::Asterisk,
            Token::Integer(10),
            Token::Illegal,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for want in tests {
            let got = lexer.next_token();
            assert_eq!(want, got);
        }
    }
}
