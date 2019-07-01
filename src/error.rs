use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum ParseError {
    FoundIllegalToken,
    FoundUnexpectedToken,
    FoundUnterminatedParentheses,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::FoundIllegalToken => write!(f, "Found an illegal token."),
            ParseError::FoundUnexpectedToken => write!(f, "Found an unexpected token."),
            ParseError::FoundUnterminatedParentheses => {
                write!(f, "Found an unterminated parentheses.")
            }
        }
    }
}
