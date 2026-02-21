use std::io::{self, Write};

use nash_parser::parser;

fn main() -> io::Result<()> {
    print!("input> ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let exprs = match parser::parse(&line) {
        Ok(exprs) => exprs,
        Err(e) => {
            eprintln!("parse error: {e:?}");
            return Ok(());
        }
    };

    println!("expressions:\n{exprs:#?}");
    Ok(())
}
