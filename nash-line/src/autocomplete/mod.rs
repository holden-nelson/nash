pub mod executable;
pub mod trie;

pub trait Completer {
    fn complete(&self, s: &str) -> Vec<String>;
}
