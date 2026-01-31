use crate::lexer;

#[derive(Debug)]
pub enum Expression {
    List(Vec<Expression>),
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Literal(String),
    Symbol(String),
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedAtom { got: lexer::Token },
    ExpectedExpression,
    ExpectedClosed,
    UnexpectedClosed,
}

pub fn parse(input: Vec<lexer::Token>) -> Result<Vec<Expression>, ParseError> {
    let mut rest: &[lexer::Token] = &input;
    let mut expressions = Vec::new();

    while !rest.is_empty() {
        expressions.push(parse_expr(&mut rest)?);
    }

    Ok(expressions)
}

fn peek(input: &[lexer::Token]) -> Option<&lexer::Token> {
    input.first()
}

fn next(input: &mut &[lexer::Token]) -> Option<lexer::Token> {
    let (first, rest) = input.split_first()?;
    *input = rest;
    Some(first.clone())
}

fn parse_expr(input: &mut &[lexer::Token]) -> Result<Expression, ParseError> {
    let token = next(input).ok_or(ParseError::ExpectedExpression)?;
    match token {
        lexer::Token::Open => Ok(Expression::List(parse_list(input)?)),
        lexer::Token::Symbol(_) | lexer::Token::Literal(_) => {
            Ok(Expression::Atom(parse_atom(token)?))
        }
        lexer::Token::Closed => Err(ParseError::UnexpectedClosed),
    }
}

fn parse_list(input: &mut &[lexer::Token]) -> Result<Vec<Expression>, ParseError> {
    let mut expressions = vec![];

    loop {
        match peek(input) {
            None => return Err(ParseError::ExpectedClosed),
            Some(lexer::Token::Closed) => {
                _ = next(input); // consume ')'
                break;
            }
            Some(_) => expressions.push(parse_expr(input)?),
        }
    }

    Ok(expressions)
}

fn parse_atom(token: lexer::Token) -> Result<Atom, ParseError> {
    match token {
        lexer::Token::Literal(s) => Ok(Atom::Literal(s)),
        lexer::Token::Symbol(s) => Ok(Atom::Symbol(s)),
        got => Err(ParseError::ExpectedAtom { got }),
    }
}
