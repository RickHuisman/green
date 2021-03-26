use std::str::{CharIndices, FromStr};
use crate::scanner::token::{Token, TokenType, Position, Keyword};
use crate::scanner::error::SyntaxError;
use crate::scanner::peek::PeekWithNext;

type Result<T> = std::result::Result<T, SyntaxError>;

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekWithNext<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn parse(source: &str) -> Result<Vec<Token>> {
        let chars = PeekWithNext::new(source.char_indices());
        let mut lexer = Lexer { source, chars, line: 1 };

        let mut tokens = vec![];
        while !lexer.is_at_end() {
            if let Some(token) = lexer.read_token() {
                tokens.push(token?);
            } else {
                break;
            }
        }

        Ok(tokens)
    }

    fn read_token(&mut self) -> Option<Result<Token<'a>>> {
        self.skip_whitespace();

        let c = self.advance();
        if c.is_none() {
            if self.is_at_end() {
                return Some(Ok(self.eof()));
            }
            return None
        }
        let (start, char) = c?;

        if char.is_alphabetic() {
            return Some(self.identifier(start));
        }

        if char.is_digit(10) {
            return Some(self.number(start));
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
                match self.string() {
                    Ok(ty) => ty,
                    Err(err) => return Some(Err(err)),
                }
            },
            '#' => {
                // '#' indicates a comment.
                self.advance_while(|&c| c != '\n');
                self.advance();
                TokenType::LineComment
            }
            _ => {
                return Some(Err(SyntaxError::UnexpectedChar(char)));
            },
        };
        Some(Ok(self.make_token(start, token_type)))
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
        if self.peek().unwrap() == '.' &&
            self.peek_next().unwrap().is_digit(10) {
            // Consume the '.'
            self.advance();

            while self.peek().unwrap().is_digit(10) {
                self.advance();
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
    use crate::scanner::token::{Token, TokenType, Position};

    #[test]
    fn it_works() {
        let input = r#"
        do
            print(10)
        end
"#;

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn parse_var2() {
        let input = r#"
        var x = 10
"#;

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn parse_var() {
        let input = "var x = if y == 10 then true else false";

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_string() {
        let expect = vec![
            Token::new(
                TokenType::String,
                "\"Hello, World\"",
                Position::new(0, 0, 1)
            ),
            Token::new(TokenType::EOF, "", Position::new(0,0,0)),
        ];
        let input = "\"Hello, World\"";

        let tokens = Lexer::parse(input);

        assert_eq!(expect, tokens);
    }

    #[test]
    fn parse_if_else() {
        let input = r#"
        if 10 > 5 then
            print(5)
        else
            print(10)
"#;

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn parse_comments() {
        let input = r#"
        # Comment
        print(10)
"#;

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
