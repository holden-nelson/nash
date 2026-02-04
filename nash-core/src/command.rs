use crate::interpret::Executable;
use std::process::Command as StdCommand;

#[derive(Debug, Default)]
pub struct Command {
    pub argv: Vec<Clause>,
}

#[derive(Debug)]
pub enum Clause {
    Literal(String),
    Bare(String),
    Embedded(Executable),
}

#[derive(Debug)]
pub enum ExecutionResult {
    Success {
        out: Vec<u8>,
        err: Vec<u8>,
        code: i32,
    },
    Error(String),
}

impl Command {
    pub fn new() -> Self {
        Command { argv: vec![] }
    }

    pub fn execute(self) -> ExecutionResult {
        if self.argv.is_empty() {
            return ExecutionResult::Success {
                out: vec![],
                err: vec![],
                code: 0,
            };
        }

        let mut argv = self.argv;
        let first = argv.remove(0);

        let program = match first {
            Clause::Bare(s) | Clause::Literal(s) => s,
            Clause::Embedded(e) => match e {
                Executable::Command { command } => match command.execute() {
                    ExecutionResult::Success { out, .. } => {
                        // Convert bytes to string
                        let output_str = String::from_utf8_lossy(&out);
                        let words: Vec<&str> = output_str.split_whitespace().collect();

                        if words.is_empty() {
                            return ExecutionResult::Error("No command specified".to_string());
                        }

                        // Splice remaining words as arguments
                        argv.splice(
                            0..0,
                            words[1..].iter().map(|&s| Clause::Literal(s.to_string())),
                        );

                        words[0].to_string()
                    }
                    ExecutionResult::Error(err) => {
                        return ExecutionResult::Error(format!("Embedded command failed: {}", err));
                    }
                },
            },
        };

        dbg!(&program, &argv);

        let mut cmd = StdCommand::new(&program);

        for clause in argv {
            match clause {
                Clause::Bare(s) | Clause::Literal(s) => {
                    cmd.arg(s);
                }
                Clause::Embedded(exec) => match exec {
                    Executable::Command { command } => match command.execute() {
                        ExecutionResult::Success { out, .. } => {
                            // Convert bytes to string and split by lines
                            let output_str = String::from_utf8_lossy(&out);
                            for line in output_str.lines() {
                                cmd.arg(line.trim());
                            }
                        }
                        ExecutionResult::Error(e) => {
                            return ExecutionResult::Error(format!(
                                "Embedded command failed: {}",
                                e
                            ));
                        }
                    },
                },
            }
        }

        match cmd.output() {
            Ok(output) => ExecutionResult::Success {
                out: output.stdout,
                err: output.stderr,
                code: output.status.code().unwrap_or(-1),
            },
            Err(e) => ExecutionResult::Error(format!("Failed to execute command: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_empty_command() {
        let cmd = Command::new();
        let result = cmd.execute();
        match result {
            ExecutionResult::Success { out, code, err } => {
                assert_eq!(out, Vec::<u8>::new());
                assert_eq!(err, Vec::<u8>::new());
                assert_eq!(code, 0);
            }
            _ => panic!("Expected success for empty command"),
        }
    }

    #[test]
    fn test_execute_simple_literal_command() {
        let mut cmd = Command::new();
        cmd.argv.push(Clause::Literal("echo".to_string()));
        cmd.argv.push(Clause::Literal("hello".to_string()));

        let result = cmd.execute();
        match result {
            ExecutionResult::Success { out, code, .. } => {
                let output = String::from_utf8_lossy(&out);
                assert!(output.contains("hello"));
                assert_eq!(code, 0);
            }
            _ => panic!("Expected successful execution"),
        }
    }

    #[test]
    fn test_execute_bare_command() {
        let mut cmd = Command::new();
        cmd.argv.push(Clause::Bare("echo".to_string()));
        cmd.argv.push(Clause::Bare("test".to_string()));

        let result = cmd.execute();
        match result {
            ExecutionResult::Success { out, code, .. } => {
                let output = String::from_utf8_lossy(&out);
                assert!(output.contains("test"));
                assert_eq!(code, 0);
            }
            _ => panic!("Expected successful execution"),
        }
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let mut cmd = Command::new();
        cmd.argv
            .push(Clause::Literal("nonexistent_command_xyz".to_string()));

        let result = cmd.execute();
        match result {
            ExecutionResult::Error(msg) => assert!(msg.contains("Failed to execute")),
            _ => panic!("Expected error for nonexistent command"),
        }
    }

    #[test]
    fn test_execute_command_with_multiple_args() {
        let mut cmd = Command::new();
        cmd.argv.push(Clause::Literal("echo".to_string()));
        cmd.argv.push(Clause::Literal("one".to_string()));
        cmd.argv.push(Clause::Literal("two".to_string()));
        cmd.argv.push(Clause::Literal("three".to_string()));

        let result = cmd.execute();
        match result {
            ExecutionResult::Success { out, .. } => {
                let output = String::from_utf8_lossy(&out);
                assert!(output.contains("one"));
                assert!(output.contains("two"));
                assert!(output.contains("three"));
            }
            _ => panic!("Expected successful execution"),
        }
    }
}
