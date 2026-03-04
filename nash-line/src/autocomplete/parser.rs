use nash_parser::lexer::{self, LexError, Token};

pub enum TokenType {
    Empty,
    Literal,
    Comment,
    Escape,
    Symbol(String),
}

pub fn determine_token_type(input: &str) -> TokenType {
    match lexer::lex(input) {
        Err(e) => match e {
            LexError::UnterminatedLiteral => {
                return TokenType::Literal;
            }
            LexError::OpenEscape => {
                return TokenType::Escape;
            }
        },
        Ok(tokens) => {
            if let Some(token) = tokens.last() {
                match token {
                    Token::Open | Token::Closed => {
                        return TokenType::Empty;
                    }
                    Token::Literal(_) => {
                        return TokenType::Literal;
                    }
                    Token::Symbol(s) => return TokenType::Symbol(s.clone()),
                }
            }
            return TokenType::Empty;
        }
    }
}
