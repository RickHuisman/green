#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Token<'a> {
        Token { token_type, source, position }
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
    Comment,

    // Literals
    String,
    Number,

    Identifier,

    Line,
    EOF,
}