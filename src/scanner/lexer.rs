use std::str::{CharIndices, FromStr};
use std::iter::Peekable;
use crate::scanner::token::{Token, TokenType, Position, Keyword};
use crate::scanner::error::SyntaxError;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn parse(source: &str) -> Vec<Token> {
        let chars = source.char_indices().peekable();
        let mut lexer = Lexer { source, chars, line: 1 };

        let mut tokens = vec![];
        while !lexer.is_at_end() {
            if let Some(token) = lexer.read_token() {
                tokens.push(token.unwrap());
            } else {
                break;
            }
        }

        tokens
    }

    fn read_token(&mut self) -> Option<Result<Token<'a>, SyntaxError>> {
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
            return Some(Ok(self.identifier(start)));
        }

        if char.is_digit(10) {
            return Some(Ok(self.number(start)));
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
            ';' | '\n' | '\r' => {
                self.skip_lines();
                TokenType::Line
            },
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
            _ => {
                return Some(Err(SyntaxError::UnexpectedChar(char)));
            },
        };
        Some(Ok(self.make_token(start, token_type)))
    }

    fn identifier(&mut self, start: usize) -> Token<'a> {
        self.advance_while(|&c| c.is_alphanumeric());

        let word = self.token_contents(start);

        let token_type = Keyword::from_str(word)
            .map(TokenType::Keyword)
            .unwrap_or(TokenType::Identifier);

        self.make_token(start, token_type)
    }

    fn number(&mut self, start: usize) -> Token<'a> {
        loop {
            if let Some(peeked) = self.peek() {
                if peeked.1.is_digit(10) {
                    self.advance().unwrap();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if let Some((_, '.')) = self.peek() {
            // TODO peekNext
            self.advance().unwrap();

            loop {
                if let Some(peeked) = self.peek() {
                    if peeked.1.is_digit(10) {
                        self.advance().unwrap();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            };
        };

        self.make_token(start, TokenType::Number)
    }

    fn string(&mut self) -> Result<TokenType, SyntaxError> {
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

    fn skip_lines(&mut self) {
        self.advance_while(|&c| c == ';' || c == '\n' || c == '\r');
    }

    fn eof(&mut self) -> Token<'a> {
        self.make_token(self.source.len(), TokenType::EOF)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek().unwrap().1 != expected {
            return false;
        }

        self.advance();

        true
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
        while let Some((_, char)) = self.peek() {
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
        self.peek().unwrap().1 == c
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
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
    fn collapse_lines_into_line() {
        let input = r#"



        "#;

        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    // #[test]
    // fn test_number() {
    //     let expect = vec![
    //         Token::new(TokenType::Number(5.0), "5", Position::new(0, 0, 0)),
    //         Token::new(TokenType::Number(10.0), "10", Position::new(0, 0, 0)),
    //         Token::new(TokenType::Number(2.0), "2", Position::new(0, 0, 0)),
    //         Token::new(TokenType::EOF, "", Position::new(0, 0, 0)),
    //     ];
    //     let input = "5 10 2";
    //
    //     let tokens = Lexer::parse(input);
    //
    //     assert_eq!(expect, tokens);
    // }

    #[test]
    fn it_works() {
        let input = r#"do
            10 + 5
        end
        "#;

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

//
//     #[test]
//     fn break_statement() {
//         let expect = vec![
//             Token::new(TokenType::Keyword(While), "while".to_string()),
//             Token::new(TokenType::LeftParen, "(".to_string()),
//             Token::new(TokenType::Keyword(True), "true".to_string()),
//             Token::new(TokenType::RightParen, ")".to_string()),
//             Token::new(TokenType::LeftBrace, "{".to_string()),
//             Token::new(TokenType::Keyword(Break), "break".to_string()),
//             Token::new(TokenType::Semicolon, ";".to_string()),
//             Token::new(TokenType::RightBrace, "}".to_string()),
//             Token::new(TokenType::EOF, "".to_string()),
//         ];
//         let input = r#"
//         while (true) {
//             break;
//         }
//         "#;
//
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.parse();
//
//         assert_eq!(expect, tokens);
//     }
//
//     #[test]
//     fn test_block() {
//         let expect = vec![
//             Token::new(TokenType::LeftBrace, "{".to_string()),
//             Token::new(TokenType::Keyword(Keyword::Var), "var".to_string()),
//             Token::new(TokenType::Identifier, "x".to_string()),
//             Token::new(TokenType::Equal, "=".to_string()),
//             Token::new(TokenType::Number(10.0), "10".to_string()),
//             Token::new(TokenType::Semicolon, ";".to_string()),
//             Token::new(TokenType::RightBrace, "}".to_string()),
//             Token::new(TokenType::Keyword(Keyword::Print), "print".to_string()),
//             Token::new(TokenType::Identifier, "x".to_string()),
//             Token::new(TokenType::Semicolon, ";".to_string()),
//             Token::new(TokenType::EOF, "".to_string()),
//         ];
//         let input = r#"
//         {
//             var x = 10;
//         }
//         print x;
//         "#;
//
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.parse();
//
//         assert_eq!(expect, tokens);
//     }
//
//     #[test]
//     fn test_modulo() {
//         let expect = vec![
//             Token::new(TokenType::Number(10.0), "10".to_string()),
//             Token::new(TokenType::Percent, "%".to_string()),
//             Token::new(TokenType::Number(3.0), "3".to_string()),
//             Token::new(TokenType::EOF, "".to_string()),
//         ];
//         let input = "10 % 3";
//
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.parse();
//
//         assert_eq!(expect, tokens);
//     }
//
//     #[test]
//     fn unclosed_string() {
//         let input = "\"Hello, World!\"";
//         let mut lexer = Lexer::new(input);
//
//         match lexer.next().unwrap().unwrap_err() {
//             SyntaxError::UnterminatedString => (),
//             _ => panic!("Expected SyntaxError::UnterminatedString"),
//         };
//     }
}