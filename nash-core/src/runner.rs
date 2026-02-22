pub mod executable;

use std::io::{self, Write};

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

#[derive(Clone)]
pub enum RunKind {
    Interactive,
    Embedded,
}

#[derive(Clone)]
pub struct RunContext {
    pub kind: RunKind,
}

impl RunContext {
    pub fn as_embedded(&self) -> Self {
        let mut ctx = self.clone();
        ctx.kind = RunKind::Embedded;
        ctx
    }
}

impl Default for RunContext {
    fn default() -> Self {
        Self {
            kind: RunKind::Interactive,
        }
    }
}

pub struct SuccessfulRun {
    out: Vec<u8>,
    err: Vec<u8>,
    code: i32,
}

impl Runnable {
    pub fn run(self) -> io::Result<SuccessfulRun> {
        self.run_in_context(RunContext::default())
    }

    pub fn run_in_context(self, ctx: RunContext) -> io::Result<SuccessfulRun> {
        match self {
            Runnable::Command { command } => command.execute(ctx),
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
        let result = runnable.run()?;
        io::stdout().write_all(&result.out)?;
        io::stderr().write_all(&result.err)?;
        io::stdout().flush()?;
    }

    Ok(())
}
