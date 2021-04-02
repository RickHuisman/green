use std::str::{CharIndices, FromStr};
use crate::syntax::token::{Token, TokenType, Position, Keyword};
use crate::syntax::error::SyntaxError;
use crate::syntax::peek::PeekWithNext;

type Result<T> = std::result::Result<T, SyntaxError>;

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekWithNext<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        let chars = PeekWithNext::new(source.char_indices());
        Lexer { source, chars, line: 1 }
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
        if c.is_none() { // TODO ???
            if self.is_at_end() {
                return Ok(self.eof())
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
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '%' => TokenType::Percent,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            ':' => TokenType::Colon,
            ';' | '\n' | '\r' => TokenType::Line,
            '!' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            },
            '=' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            },
            '<' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::LessThanEqual
                } else {
                    TokenType::LessThan
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.advance();
                    TokenType::GreaterThanEqual
                } else {
                    TokenType::GreaterThan
                }
            },
            '"' => {
                return match self.string() {
                    Ok(ty) => {
                        let source = self.token_contents(start).trim_matches('"');

                        let mut token = self.make_token(start, ty);
                        token.source = source;
                        Ok(token)
                    },
                    Err(err) => Err(err),
                }
            },
            '#' => {
                // '#' indicates a comment.
                self.advance_while(|&c| c != '\n');
                self.advance();
                TokenType::LineComment
            }
            _ => {
                return Err(SyntaxError::UnexpectedChar(char));
            },
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
        let position = Position::new(
            start,
            start + source.len(),
            self.line
        );
        Token::new(token_type, source, position)
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(|&c| c == ' ' || c == '\t');
    }

    fn eof(&mut self) -> Token<'a> {
        self.make_token(self.source.len(), TokenType::EOF)
    }

    fn token_contents(&mut self, start: usize) -> &'a str {
        let end = self.chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.source.len());
        &self.source[start..end].trim_end()
    }

    fn advance_while<F>(&mut self, f: F) -> usize where for<'r> F: Fn(&'r char,) -> bool {
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

    fn match_next(&mut self, c: char) -> bool { // TODO Option???
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
    use crate::syntax::token::{Token, TokenType, Position};

    #[test]
    fn parse_number() {
        let expect = vec![
            Token::new(TokenType::Number, "2", Position::new(
                0, 0, 1
            )),
            Token::new(TokenType::Number, "10", Position::new(
                2, 3, 1
            )),
            Token::new(TokenType::Number, "3.33", Position::new(
                5, 8, 1
            )),
        ];

        let input = "2 10 3.33";

        let actual = Lexer::parse(input).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_string() {
        let expect = vec![
            Token::new(TokenType::String, "foo", Position::new(
                0, 0, 1
            )),
            Token::new(TokenType::String, "bar", Position::new(
                2, 3, 1
            )),
        ];

        let input = r#""foo" "bar""#;

        let actual = Lexer::parse(input).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_def() {
        let input = r#"
        def double(x: Int) -> Int
        end
        "#;

        let actual = Lexer::parse(input).unwrap();
        for token in actual {
            println!("{:?}", token);
        }
    }

    #[test]
    fn parse_import() {
        let input = r#"
        import foo.bar
        "#;

        let actual = Lexer::parse(input).unwrap();
        for token in actual {
            println!("{:?}", token);
        }
    }

    #[test]
    fn parse_for() {
        let input = r#"
        for x in 1 to 10 do
        end
        "#;

        let actual = Lexer::parse(input).unwrap();
        for token in actual {
            println!("{:?}", token);
        }
    }
}
