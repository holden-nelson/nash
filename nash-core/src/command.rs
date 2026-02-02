use crate::interpret::Executable;
use std::process::Command as StdCommand;

#[derive(Debug)]
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
    Success { out: String, err: String, code: i32 },
    Error(String),
}

impl Command {
    pub fn new() -> Self {
        Command { argv: vec![] }
    }

    pub fn execute(self) -> ExecutionResult {
        if self.argv.is_empty() {
            return ExecutionResult::Error("No command specified".to_string());
        }

        let mut argv = self.argv.into_iter();
        let first = match argv.next() {
            Some(c) => c,
            None => return ExecutionResult::Error("No command specified".to_string()),
        };

        let program = match first {
            Clause::Bare(s) | Clause::Literal(s) => s,
            Clause::Embedded(e) => match e {
                Executable::Command { command } => match command.execute() {
                    ExecutionResult::Success { out, .. } => out.trim().to_string(),
                    ExecutionResult::Error(err) => {
                        return ExecutionResult::Error(format!("Embedded command failed: {}", err));
                    }
                },
            },
        };

        let mut cmd = StdCommand::new(&program);

        for clause in argv {
            match clause {
                Clause::Bare(s) | Clause::Literal(s) => {
                    cmd.arg(s);
                }
                Clause::Embedded(exec) => match exec {
                    Executable::Command { command } => match command.execute() {
                        ExecutionResult::Success { out, .. } => {
                            for line in out.lines() {
                                cmd.arg(line);
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
            Ok(output) => {
                let out = String::from_utf8_lossy(&output.stdout).to_string();
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                let code = output.status.code().unwrap_or(-1);

                ExecutionResult::Success { out, err, code }
            }
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
            ExecutionResult::Error(msg) => assert_eq!(msg, "No command specified"),
            _ => panic!("Expected error for empty command"),
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
                assert!(out.contains("hello"));
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
                assert!(out.contains("test"));
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
                assert!(out.contains("one"));
                assert!(out.contains("two"));
                assert!(out.contains("three"));
            }
            _ => panic!("Expected successful execution"),
        }
    }
}
