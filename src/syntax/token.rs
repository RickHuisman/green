use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Token<'a> {
        Token {
            token_type,
            source,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    start: usize,
    end: usize,
    line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Position { start, end, line }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Percent,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Slash,
    Colon,

    // Literals
    String,
    Number,

    // Keywords
    Keyword(Keyword),
    Identifier,

    Comment,
    LineComment,
    Line,

    EOF,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Keyword {
    Import,
    While,
    For,
    In,
    To,
    DownTo,
    Step,
    Print,
    Do,
    End,
    Def,
    Var,
    Val,
    If,
    Else,
    Then,
    True,
    False,
    Return,
    Struct,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        match word {
            "import" => Ok(Keyword::Import),
            "while" => Ok(Keyword::While),
            "for" => Ok(Keyword::For),
            "in" => Ok(Keyword::In),
            "to" => Ok(Keyword::To),
            "downTo" => Ok(Keyword::DownTo),
            "step" => Ok(Keyword::Step),
            "print" => Ok(Keyword::Print),
            "do" => Ok(Keyword::Do),
            "end" => Ok(Keyword::End),
            "def" => Ok(Keyword::Def),
            "var" => Ok(Keyword::Var),
            "val" => Ok(Keyword::Val),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "then" => Ok(Keyword::Then),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "return" => Ok(Keyword::Return),
            "struct" => Ok(Keyword::Struct),
            _ => Err(()),
        }
    }
}
