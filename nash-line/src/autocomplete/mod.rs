use crate::autocomplete::{
    executable::ExecutableCompleter,
    parser::{TokenType, determine_token_type},
};

pub mod executable;
mod parser;
pub mod trie;

pub trait Completer {
    fn complete(&self, s: &str) -> Vec<String>;
}

pub fn autocomplete(buffer_fragment: &str) -> Vec<String> {
    let completer = ExecutableCompleter::new().with_executables_in_path();

    match determine_token_type(buffer_fragment) {
        TokenType::Symbol(s) => {
            return completer.complete(&s);
        }
        _ => {
            return vec![];
        }
    }
}
