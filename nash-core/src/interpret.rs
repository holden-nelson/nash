use crate::command::{Clause, Command};
use nash_parser::parser::{Atom, Expression};

#[derive(Debug)]
pub enum Executable {
    Command { command: Command },
    // coming soon
    // Pipeline
    // Binding
    // Logical operators
    // ...
}

#[derive(Debug)]
pub enum InterpretError {
    TopLevelAtom,
}

pub fn interpret(expressions: Vec<Expression>) -> Result<Vec<Executable>, InterpretError> {
    let mut executables = vec![];

    for e in expressions {
        match e {
            Expression::List(v) => executables.push(interpret_list(v)?),
            Expression::Atom(_) => Err(InterpretError::TopLevelAtom)?,
        }
    }

    Ok(executables)
}

pub fn interpret_list(expressions: Vec<Expression>) -> Result<Executable, InterpretError> {
    let operator = expressions.first();

    match operator {
        _ => interpret_command(expressions),
    }
}

pub fn interpret_command(expressions: Vec<Expression>) -> Result<Executable, InterpretError> {
    let mut argv = vec![];

    for e in expressions {
        match e {
            Expression::Atom(a) => match a {
                Atom::Literal(s) => argv.push(Clause::Literal(s)),
                Atom::Symbol(s) => argv.push(Clause::Bare(s)),
            },
            Expression::List(v) => {
                let executable = interpret_list(v)?;
                argv.push(Clause::Embedded(executable));
            }
        }
    }

    Ok(Executable::Command {
        command: Command { argv },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nash_parser::parser::{Atom, Expression};

    #[test]
    fn test_simple_command() {
        let expressions = vec![Expression::List(vec![
            Expression::Atom(Atom::Symbol("ls".to_string())),
            Expression::Atom(Atom::Symbol("-la".to_string())),
        ])];

        let result = interpret(expressions).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0] {
            Executable::Command { command } => {
                assert_eq!(command.argv.len(), 2);
                match &command.argv[0] {
                    Clause::Bare(s) => assert_eq!(s, "ls"),
                    _ => panic!("Expected Bare clause"),
                }
                match &command.argv[1] {
                    Clause::Bare(s) => assert_eq!(s, "-la"),
                    _ => panic!("Expected Bare clause"),
                }
            }
        }
    }

    #[test]
    fn test_command_with_literal() {
        let expressions = vec![Expression::List(vec![
            Expression::Atom(Atom::Symbol("echo".to_string())),
            Expression::Atom(Atom::Literal("hello world".to_string())),
        ])];

        let result = interpret(expressions).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0] {
            Executable::Command { command } => {
                assert_eq!(command.argv.len(), 2);
                match &command.argv[1] {
                    Clause::Literal(s) => assert_eq!(s, "hello world"),
                    _ => panic!("Expected Literal clause"),
                }
            }
        }
    }

    #[test]
    fn test_nested_command() {
        let expressions = vec![Expression::List(vec![
            Expression::Atom(Atom::Symbol("ls".to_string())),
            Expression::Atom(Atom::Symbol("-la".to_string())),
            Expression::List(vec![
                Expression::Atom(Atom::Symbol("cat".to_string())),
                Expression::Atom(Atom::Symbol("file.txt".to_string())),
            ]),
        ])];

        let result = interpret(expressions).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0] {
            Executable::Command { command } => {
                assert_eq!(command.argv.len(), 3);
                match &command.argv[2] {
                    Clause::Embedded(exec) => match exec {
                        Executable::Command {
                            command: inner_command,
                        } => {
                            assert_eq!(inner_command.argv.len(), 2);
                            match &inner_command.argv[0] {
                                Clause::Bare(s) => assert_eq!(s, "cat"),
                                _ => panic!("Expected Bare clause"),
                            }
                        }
                    },
                    _ => panic!("Expected Embedded clause"),
                }
            }
        }
    }

    #[test]
    fn test_multiple_top_level_commands() {
        let expressions = vec![
            Expression::List(vec![Expression::Atom(Atom::Symbol("pwd".to_string()))]),
            Expression::List(vec![Expression::Atom(Atom::Symbol("ls".to_string()))]),
        ];

        let result = interpret(expressions).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_top_level_atom_error() {
        let expressions = vec![Expression::Atom(Atom::Symbol("not-a-list".to_string()))];

        let result = interpret(expressions);
        assert!(result.is_err());
        match result {
            Err(InterpretError::TopLevelAtom) => {}
            _ => panic!("Expected TopLevelAtom error"),
        }
    }

    #[test]
    fn test_deeply_nested_commands() {
        let expressions = vec![Expression::List(vec![
            Expression::Atom(Atom::Symbol("echo".to_string())),
            Expression::List(vec![
                Expression::Atom(Atom::Symbol("cat".to_string())),
                Expression::List(vec![
                    Expression::Atom(Atom::Symbol("ls".to_string())),
                    Expression::Atom(Atom::Symbol("/tmp".to_string())),
                ]),
            ]),
        ])];

        let result = interpret(expressions).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0] {
            Executable::Command { command } => {
                assert_eq!(command.argv.len(), 2);
                // Verify nested structure exists
                match &command.argv[1] {
                    Clause::Embedded(_) => {}
                    _ => panic!("Expected Embedded clause at depth 1"),
                }
            }
        }
    }
}
