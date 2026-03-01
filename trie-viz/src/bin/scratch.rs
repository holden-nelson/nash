use nash_line::autocomplete::Trie;

fn main() {
    let mut trie = Trie::new();

    // A small set of words to demonstrate the trie structure.
    // Edit these freely — then run `cargo run --bin scratch -p trie-viz`
    // and open the URL that gets printed to stderr.
    let words = [
        "fooBar", "foobaz", "fooqUx", "cat", "cargo", "car", "card", "care",
    ];

    for word in words {
        trie.insert(word);
    }

    trie_viz::log_playground_url(&trie);
}
