use nash_line::autocomplete::executable::ExecutableCompleter;

fn main() {
    let completer = ExecutableCompleter::new().with_executables_in_path();

    let trie = completer.trie();

    trie_viz::log_playground_url(&trie);
}
