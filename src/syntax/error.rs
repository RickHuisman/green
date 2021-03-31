use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum SyntaxError {
    UnexpectedEOF,
    UnexpectedChar(char),
    UnterminatedString,
    ExpectAfter(&'static str, &'static str),
    ExpectBefore(&'static str, &'static str),
    Expect(&'static str),
    InvalidAssignment,
    TooManyArguments,
    TooManyParameters,
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedEOF => write!(f, "Unexpected end of input"),
            SyntaxError::UnexpectedChar(char) => write!(f, "unexpected character {}", char),
            SyntaxError::UnterminatedString => write!(f, "Unterminated string."),
            SyntaxError::ExpectAfter(e1, e2) => write!(f, "Expect {} after {}", e1, e2),
            SyntaxError::ExpectBefore(e1, e2) => write!(f, "Expect {} befor {}", e1, e2),
            SyntaxError::Expect(e) => write!(f, "Expect {}", e),
            SyntaxError::InvalidAssignment => write!(f, "Invalid assignment target."),
            SyntaxError::TooManyArguments => write!(f, "Cannot have more than 8 arguments."),
            SyntaxError::TooManyParameters => write!(f, "Cannot have more than 8 parameters."),
        }
    }
}
