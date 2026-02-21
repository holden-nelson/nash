use crate::runner::{Runnable, SuccessfulRun};
use std::{io, process::Command as StdCommand};

#[derive(Debug, Default)]
pub struct Executable {
    pub argv: Vec<Clause>,
}

#[derive(Debug)]
pub enum Clause {
    Literal(String),
    Bare(String),
    Embedded(Runnable),
}

impl Executable {
    pub fn new() -> Self {
        Executable { argv: vec![] }
    }

    pub fn execute(self) -> io::Result<SuccessfulRun> {
        if self.argv.is_empty() {
            return Ok(SuccessfulRun {
                out: vec![],
                err: vec![],
                code: 0,
            });
        }

        let mut argv = self.argv;
        let first = argv.remove(0);

        let program = match first {
            Clause::Bare(s) | Clause::Literal(s) => s,
            Clause::Embedded(runnable) => {
                let result = runnable.run()?;

                let output_str = String::from_utf8_lossy(&result.out);
                let mut parts = output_str.split_whitespace();

                let program = parts.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "No command specified")
                })?;

                // Remaining words become leading args
                argv.splice(0..0, parts.map(|s| Clause::Literal(s.to_string())));

                program.to_string()
            }
        };

        let mut cmd = StdCommand::new(&program);

        for clause in argv {
            match clause {
                Clause::Bare(s) | Clause::Literal(s) => {
                    cmd.arg(s);
                }
                Clause::Embedded(runnable) => {
                    let result = runnable.run()?;
                    let output_str = String::from_utf8_lossy(&result.out);
                    for line in output_str.lines() {
                        cmd.arg(line.trim());
                    }
                }
            }
        }

        let output = cmd.output()?;

        Ok(SuccessfulRun {
            out: output.stdout,
            err: output.stderr,
            code: output.status.code().unwrap_or(-1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_execute_empty_command() {
        let cmd = Executable::new();
        let result = cmd.execute();

        let SuccessfulRun { out, err, code } = result.expect("Expected Ok for empty command");
        assert_eq!(out, Vec::<u8>::new());
        assert_eq!(err, Vec::<u8>::new());
        assert_eq!(code, 0);
    }

    #[test]
    fn test_execute_simple_literal_command() {
        let mut cmd = Executable::new();
        cmd.argv.push(Clause::Literal("echo".to_string()));
        cmd.argv.push(Clause::Literal("hello".to_string()));

        let SuccessfulRun { out, code, .. } = cmd.execute().expect("Expected successful execution");
        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("hello"));
        assert_eq!(code, 0);
    }

    #[test]
    fn test_execute_bare_command() {
        let mut cmd = Executable::new();
        cmd.argv.push(Clause::Bare("echo".to_string()));
        cmd.argv.push(Clause::Bare("test".to_string()));

        let SuccessfulRun { out, code, .. } = cmd.execute().expect("Expected successful execution");
        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("test"));
        assert_eq!(code, 0);
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let mut cmd = Executable::new();
        cmd.argv
            .push(Clause::Literal("nonexistent_command_xyz".to_string()));

        let result = cmd.execute();
        match result {
            Ok(_) => panic!("Expected error for nonexistent command"),
            Err(e) => {
                // On Unix/macOS this is typically NotFound.
                assert_eq!(e.kind(), io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn test_execute_command_with_multiple_args() {
        let mut cmd = Executable::new();
        cmd.argv.push(Clause::Literal("echo".to_string()));
        cmd.argv.push(Clause::Literal("one".to_string()));
        cmd.argv.push(Clause::Literal("two".to_string()));
        cmd.argv.push(Clause::Literal("three".to_string()));

        let SuccessfulRun { out, .. } = cmd.execute().expect("Expected successful execution");
        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("one"));
        assert!(output.contains("two"));
        assert!(output.contains("three"));
    }
}
