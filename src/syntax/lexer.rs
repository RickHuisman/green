use crate::error::SyntaxError;
use crate::syntax::peek::PeekWithNext;
use crate::syntax::token::{Keyword, Position, Token, TokenType};
use std::str::{CharIndices, FromStr};

type Result<T> = std::result::Result<T, SyntaxError>;

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekWithNext<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        let chars = PeekWithNext::new(source.char_indices());
        Lexer {
            source,
            chars,
            line: 1,
        }
    }

    pub fn parse(source: &str) -> Result<Vec<Token>> {
        let mut lexer = Lexer::new(source);

        let mut tokens = vec![];
        while !lexer.is_at_end() {
            tokens.push(lexer.read_token()?);
        }

        Ok(tokens)
    }

    fn read_token(&mut self) -> Result<Token<'a>> {
        self.skip_whitespace();

        let c = self.advance();
        if c.is_none() {
            // TODO ???
            if self.is_at_end() {
                return Ok(self.eof());
            }
        }
        let (start, char) = c.unwrap();

        if char.is_alphabetic() {
            return self.identifier(start);
        }

        if char.is_digit(10) {
            return self.number(start);
        }

        let token_type = match char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => {
                if self.match_next('>') {
                    self.advance();
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            },
            '+' => TokenType::Plus,
            '%' => TokenType::Percent,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '!' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::LessThanEqual
                } else {
                    TokenType::LessThan
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::GreaterThanEqual
                } else {
                    TokenType::GreaterThan
                }
            }
            '"' => {
                return match self.string() {
                    Ok(ty) => {
                        let source = self.token_contents(start).trim_matches('"');

                        let mut token = self.make_token(start, ty);
                        token.source = source;
                        Ok(token)
                    }
                    Err(err) => Err(err),
                };
            }
            '#' => {
                // '#' indicates a comment.
                self.advance_while(|&c| c != '\n');
                self.advance();
                TokenType::LineComment
            }
            _ => {
                return Err(SyntaxError::UnexpectedChar(char));
            }
        };
        Ok(self.make_token(start, token_type))
    }

    fn identifier(&mut self, start: usize) -> Result<Token<'a>> {
        self.advance_while(|&c| c.is_alphanumeric());

        let word = self.token_contents(start);

        let token_type = Keyword::from_str(word)
            .map(TokenType::Keyword)
            .unwrap_or(TokenType::Identifier);

        Ok(self.make_token(start, token_type))
    }

    fn number(&mut self, start: usize) -> Result<Token<'a>> {
        self.advance_while(|c| c.is_digit(10));

        // Look for a fractional part
        if let Some(peek) = self.peek() {
            if peek == '.' {
                if let Some(next) = self.peek_next() {
                    if next.is_digit(10) {
                        // Consume the '.'
                        self.advance();

                        self.advance_while(|c| c.is_digit(10));
                    }
                }
            }
        }

        Ok(self.make_token(start, TokenType::Number))
    }

    fn string(&mut self) -> Result<TokenType> {
        self.advance_while(|&c| c != '"');
        if self.is_at_end() {
            return Err(SyntaxError::UnterminatedString);
        }

        // Consume the '"'
        self.advance();

        Ok(TokenType::String)
    }

    fn make_token(&mut self, start: usize, token_type: TokenType) -> Token<'a> {
        let source = self.token_contents(start);
        let position = Position::new(start, start + source.len(), self.line);
        Token::new(token_type, source, position)
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(|&c| c.is_whitespace());
    }

    fn eof(&mut self) -> Token<'a> {
        self.make_token(self.source.len(), TokenType::EOF)
    }

    fn token_contents(&mut self, start: usize) -> &'a str {
        let end = self
            .chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.source.len());
        &self.source[start..end].trim_end()
    }

    fn advance_while<F>(&mut self, f: F) -> usize
    where
        for<'r> F: Fn(&'r char) -> bool,
    {
        let mut count = 0;
        while let Some(char) = self.peek() {
            if f(&char) {
                self.advance();
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(current, c)| {
            if c == '\n' {
                self.line += 1;
            }
            (current, c)
        })
    }

    fn match_next(&mut self, c: char) -> bool {
        // TODO Option???
        self.peek().unwrap() == c
    }

    fn peek_next(&mut self) -> Option<char> {
        self.chars.peek_next().map(|&(_, c)| c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::syntax::token::{Keyword, Position, Token, TokenType};

    // TODO: Test Token position
    fn empty_pos() -> Position {
        Position::new(0, 0, 0)
    }

    #[test]
    fn parse_number() {
        let expect = vec![
            Token::new(TokenType::Number, "2", empty_pos()),
            Token::new(TokenType::Number, "10", empty_pos()),
            Token::new(TokenType::Number, "3.33", empty_pos()),
        ];

        let input = "2 10 3.33";

        let actual = Lexer::parse(input).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_string() {
        let expect = vec![
            Token::new(TokenType::String, "foo", empty_pos()),
            Token::new(TokenType::String, "bar", Position::new(2, 3, 1)),
        ];

        let input = r#""foo" "bar""#;
        let actual = Lexer::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_fn() {
        let expect = vec![
            Token::new(TokenType::Identifier, "fn", empty_pos()),
            Token::new(TokenType::Identifier, "double", empty_pos()),
            Token::new(TokenType::LeftParen, "(", empty_pos()),
            Token::new(TokenType::Identifier, "x", empty_pos()),
            Token::new(TokenType::Colon, ":", empty_pos()),
            Token::new(TokenType::Identifier, "Int", empty_pos()),
            Token::new(TokenType::RightParen, ")", empty_pos()),
            Token::new(TokenType::Arrow, "->", empty_pos()),
            Token::new(TokenType::Identifier, "Int", empty_pos()),
            Token::new(TokenType::LeftBrace, "{", empty_pos()),
            Token::new(TokenType::RightBrace, "}", empty_pos()),
            Token::new(TokenType::EOF, "", empty_pos()),
        ];

        let input = r#"
        fn double(x: Int) -> Int {
        }
        "#;
        let actual = Lexer::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_import() {
        let expect = vec![
            Token::new(TokenType::Keyword(Keyword::Import), "import", empty_pos()),
            Token::new(TokenType::Identifier, "foo", empty_pos()),
            Token::new(TokenType::Dot, ".", empty_pos()),
            Token::new(TokenType::Identifier, "bar", empty_pos()),
            Token::new(TokenType::EOF, "", empty_pos()),
        ];

        let input = r#"
        import foo.bar
        "#;
        let actual = Lexer::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_for() {
        let expect = vec![
            Token::new(TokenType::Keyword(Keyword::For), "for", empty_pos()),
            Token::new(TokenType::Identifier, "x", empty_pos()),
            Token::new(TokenType::Keyword(Keyword::In), "in", empty_pos()),
            Token::new(TokenType::Number, "1", empty_pos()),
            Token::new(TokenType::Keyword(Keyword::To), "to", empty_pos()),
            Token::new(TokenType::Number, "10", empty_pos()),
            Token::new(TokenType::LeftBrace, "{", empty_pos()),
            Token::new(TokenType::RightBrace, "}", empty_pos()),
            Token::new(TokenType::EOF, "", empty_pos()),
        ];

        let input = r#"
        for x in 1 to 10 {
        }
        "#;
        let actual = Lexer::parse(input).unwrap();

        assert_eq!(expect, actual);
    }
}
