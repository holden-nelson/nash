use nash_core::command::{Clause, Command};
use nash_core::interpret::{Executable, interpret};
use nash_parser::{lexer, parser};
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() || input == "exit" {
            break;
        }

        // Lex
        let tokens = match lexer::lex(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Lex error: {:?}", e);
                continue;
            }
        };

        // Parse
        let expressions = match parser::parse(tokens) {
            Ok(exprs) => exprs,
            Err(e) => {
                println!("Parse error: {:?}", e);
                continue;
            }
        };

        // Interpret
        let executables = match interpret(expressions) {
            Ok(execs) => execs,
            Err(e) => {
                println!("Interpret error: {:?}", e);
                continue;
            }
        };

        println!("{:#?}", executables);

        for executable in executables {
            match executable {
                Executable::Command { command } => {
                    let result = command.execute();
                    println!("Command test => {:?}", result);
                }
            }
        }
    }
}
