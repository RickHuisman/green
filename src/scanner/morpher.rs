use crate::scanner::token::{Token, TokenType};
use crate::scanner::lexer::Lexer;

/// Cleans a sequence of tokens into a token sequence of meaningful tokens.
/// Tokens that are removed from the sequence:
/// - Comments
/// - Unessential lines
pub fn morph<'a>(mut tokens: Vec<Token<'a>>) -> Vec<Token<'a>> {
    let mut test = vec![];

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token.token_type {
            TokenType::LeftParen => {}
            TokenType::RightParen => {}
            TokenType::LeftBrace => {}
            TokenType::RightBrace => {}
            TokenType::Comma => {}
            TokenType::Dot => {}
            TokenType::Minus => {}
            TokenType::Plus => {}
            TokenType::Percent => {}
            TokenType::Star => {}
            TokenType::Bang => {}
            TokenType::BangEqual => {}
            TokenType::Equal => {}
            TokenType::EqualEqual => {}
            TokenType::LessThan => {}
            TokenType::LessThanEqual => {}
            TokenType::GreaterThan => {}
            TokenType::GreaterThanEqual => {}
            TokenType::Slash => {}
            TokenType::Comment => {}
            TokenType::String => {}
            TokenType::Number => {}
            TokenType::Keyword(_) => {}
            TokenType::Identifier => {}
            TokenType::Line => {}
            TokenType::EOF => {}
            _ => test.push(token),
        }
    }

    test
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morph_comments() {
        let input = r#"
        # This is a test!
        print(10)

        "#;
        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }

    }

    #[test]
    fn morph() {
        let input = r#"

        print(10)

        "#;
        let tokens = Lexer::parse(input);
        for token in tokens {
            println!("{:?}", token);
        }

    }
}
