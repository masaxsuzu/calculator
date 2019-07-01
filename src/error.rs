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

#[derive(PartialEq, Clone, Debug)]
pub enum RuntimeError {
    DivideByZero,
    FoundNoProgram,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::DivideByZero => write!(f, "Divide a number by 0."),
            RuntimeError::FoundNoProgram => write!(f, "Found no program."),
        }
    }
}
