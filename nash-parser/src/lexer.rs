enum LexerState {
    Normal,
    Escaped,
    SingleLiteral,
    DoubleLiteral,
    SingleLiteralEscaped,
    DoubleLiteralEscaped,
    Comment,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Open,
    Symbol(String),
    Literal(String),
    Closed,
}

#[derive(Debug)]
pub enum LexError {
    UnterminatedLiteral,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut current_lex_state = LexerState::Normal;
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    // Push a token, wrapped in specified Enum
    macro_rules! push_token {
        ($constructor:expr) => {
            if !current_token.is_empty() {
                tokens.push($constructor(current_token.clone()));
                current_token.clear();
            }
        };
    }

    for c in input.chars() {
        match current_lex_state {
            LexerState::Normal => {
                if c.is_whitespace() {
                    push_token!(Token::Symbol);
                } else if c == '"' {
                    push_token!(Token::Symbol);
                    current_lex_state = LexerState::DoubleLiteral;
                } else if c == '\'' {
                    push_token!(Token::Symbol);
                    current_lex_state = LexerState::SingleLiteral;
                } else if c == '(' {
                    push_token!(Token::Symbol);
                    tokens.push(Token::Open);
                } else if c == ')' {
                    push_token!(Token::Symbol);
                    tokens.push(Token::Closed);
                } else if c == '\\' {
                    current_lex_state = LexerState::Escaped;
                } else if c == ';' {
                    current_lex_state = LexerState::Comment;
                } else {
                    current_token.push(c);
                }
            }
            LexerState::Escaped => {
                const ESCAPEDCHARS: &[char] = &['(', ')', '"', '\''];

                if ESCAPEDCHARS.contains(&c) {
                    current_token.push(c);
                } else if c.is_whitespace() {
                    current_token.push(c);
                } else if c == '\\' {
                    current_token.push('\\');
                    continue;
                } else {
                    current_token.push('\\');
                    current_token.push(c);
                }

                current_lex_state = LexerState::Normal;
            }
            LexerState::DoubleLiteral => {
                if c == '"' {
                    push_token!(Token::Literal);
                    current_lex_state = LexerState::Normal;
                } else if c == '\\' {
                    current_lex_state = LexerState::DoubleLiteralEscaped;
                } else {
                    current_token.push(c);
                }
            }
            LexerState::SingleLiteral => {
                if c == '\'' {
                    push_token!(Token::Literal);
                    current_lex_state = LexerState::Normal;
                } else if c == '\\' {
                    current_lex_state = LexerState::SingleLiteralEscaped;
                } else {
                    current_token.push(c);
                }
            }
            LexerState::DoubleLiteralEscaped => {
                if c == '"' || c == '\'' {
                    current_token.push(c);
                } else if c == '\\' {
                    current_token.push('\\');
                    continue;
                } else {
                    current_token.push('\\');
                    current_token.push(c);
                }

                current_lex_state = LexerState::DoubleLiteral;
            }
            LexerState::SingleLiteralEscaped => {
                if c == '"' || c == '\'' {
                    current_token.push(c);
                } else if c == '\\' {
                    current_token.push('\\');
                    continue;
                } else {
                    current_token.push('\\');
                    current_token.push(c);
                }

                current_lex_state = LexerState::SingleLiteral;
            }
            LexerState::Comment => {
                if c == '\n' {
                    current_lex_state = LexerState::Normal;
                }
            }
        }
    }

    match current_lex_state {
        LexerState::Normal | LexerState::Comment => {
            push_token!(Token::Symbol)
        }
        LexerState::DoubleLiteral | LexerState::SingleLiteral => {
            return Err(LexError::UnterminatedLiteral);
        }
        _ => panic!("You should never be able to end this loop in an escaped state"),
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_ok(input: &str) -> Vec<Token> {
        match lex(input) {
            Ok(tokens) => tokens,
            Err(_) => panic!("expected lex() to return Ok(...)"),
        }
    }

    #[test]
    fn test_simple_symbol() {
        assert_eq!(lex_ok("abc\n"), vec![Token::Symbol("abc".to_string())]);
    }

    #[test]
    fn test_list() {
        assert_eq!(
            lex_ok("(ls)\n"),
            vec![Token::Open, Token::Symbol("ls".to_string()), Token::Closed]
        );
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            lex_ok("(pipe (cat file) (grep err))\n"),
            vec![
                Token::Open,
                Token::Symbol("pipe".to_string()),
                Token::Open,
                Token::Symbol("cat".to_string()),
                Token::Symbol("file".to_string()),
                Token::Closed,
                Token::Open,
                Token::Symbol("grep".to_string()),
                Token::Symbol("err".to_string()),
                Token::Closed,
                Token::Closed
            ]
        );
    }

    #[test]
    fn test_escaped_paren() {
        assert_eq!(lex_ok("\\(\n"), vec![Token::Symbol("(".to_string())]);
    }

    #[test]
    fn test_escaped_backslash() {
        assert_eq!(lex_ok("\\a\n"), vec![Token::Symbol("\\a".to_string())]);
    }

    #[test]
    fn test_double_literal() {
        assert_eq!(
            lex_ok("\"hello world\"\n"),
            vec![Token::Literal("hello world".to_string())]
        );
    }

    #[test]
    fn test_single_literal() {
        assert_eq!(
            lex_ok("'hello' "),
            vec![Token::Literal("hello".to_string())]
        );
    }

    #[test]
    fn test_escaped_quote_in_literal() {
        assert_eq!(
            lex_ok("\"he\\\"llo\" "),
            vec![Token::Literal("he\"llo".to_string())]
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            lex_ok("; this is a comment\n(ls)\n"),
            vec![Token::Open, Token::Symbol("ls".to_string()), Token::Closed]
        );
    }

    #[test]
    fn test_symbol_with_space_escaped() {
        assert_eq!(
            lex_ok("file\\ name\n"),
            vec![Token::Symbol("file name".to_string())]
        );
    }

    #[test]
    fn test_mixed() {
        assert_eq!(
            lex_ok("(def x 'value')\n"),
            vec![
                Token::Open,
                Token::Symbol("def".to_string()),
                Token::Symbol("x".to_string()),
                Token::Literal("value".to_string()),
                Token::Closed
            ]
        );
    }
}
