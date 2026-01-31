use std::io::{self, Write};

use nash_parser::{lexer, parser};

fn main() -> io::Result<()> {
    print!("input> ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let tokens = match lexer::lex(&line) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("lex error: {e:?}");
            return Ok(());
        }
    };

    println!("tokens:\n{tokens:#?}");

    let exprs = match parser::parse(tokens) {
        Ok(exprs) => exprs,
        Err(e) => {
            eprintln!("parse error: {e:?}");
            return Ok(());
        }
    };

    println!("expressions:\n{exprs:#?}");
    Ok(())
}
