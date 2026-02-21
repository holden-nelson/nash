pub mod executable;

use std::io;

use executable::Executable;
use nash_parser::parser;
use thiserror::Error;

use crate::interpret::interpret;

#[derive(Debug)]
pub enum Runnable {
    Command { command: Executable },
    // coming soon
    // Pipeline
    // Binding
    // Logical operators
    // ...
}

pub struct SuccessfulRun {
    out: Vec<u8>,
    err: Vec<u8>,
    code: i32,
}

impl Runnable {
    pub fn run(self) -> io::Result<SuccessfulRun> {
        match self {
            Runnable::Command { command } => command.execute(),
        }
    }
}

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error(transparent)]
    Parse(#[from] parser::ParseError),

    #[error(transparent)]
    Interpret(#[from] crate::interpret::InterpretError),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub fn run(input: &str) -> Result<(), RunnerError> {
    let parsed = parser::parse(input)?;
    let runnables = interpret(parsed)?;
    for runnable in runnables {
        runnable.run()?;
    }

    Ok(())
}
