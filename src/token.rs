#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Integer(i64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    LeftParen,
    RightParen,
}
