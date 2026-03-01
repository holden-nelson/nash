use nash_line::autocomplete::trie::{Trie, TrieNode};
use serde::Serialize;

// Embed the HTML template at compile time so the binary is self-contained.
const HTML_TEMPLATE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/index.html"));

// ---------------------------------------------------------------------------
// Snapshot types — a serialization-friendly mirror of the Trie internals.
// These live here so that nash-line never needs to know about serde.
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct TrieNodeSnapshot {
    /// The normalized (lowercased) char on the incoming edge. `None` for root.
    edge: Option<char>,
    /// Original-cased strings that terminate exactly at this node.
    terminals: Vec<String>,
    /// Total count of terminal strings in this subtrie.
    terminals_in_subtrie: usize,
    /// Children, sorted by edge char for deterministic output.
    children: Vec<TrieNodeSnapshot>,
}

fn snapshot_node(node: &TrieNode, edge: Option<char>) -> TrieNodeSnapshot {
    let mut children: Vec<TrieNodeSnapshot> = node
        .children
        .iter()
        .map(|(&c, child)| snapshot_node(child, Some(c)))
        .collect();

    // Sort by edge char so the JSON (and therefore the URL) is deterministic
    // regardless of HashMap iteration order.
    children.sort_by_key(|n| n.edge);

    TrieNodeSnapshot {
        edge,
        terminals: node.terminals.clone(),
        terminals_in_subtrie: node.terminals_in_subtrie,
        children,
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Serialise `trie` to JSON, inject it into the HTML template, write to a
/// temp file, and print a `file://` URL to `stderr`.
///
/// Usage:
/// ```rust,ignore
/// trie_viz::log_playground_url(&my_trie);
/// ```
pub fn log_playground_url(trie: &Trie) {
    let snapshot = snapshot_node(&trie.root_node, None);

    let json = serde_json::to_string(&snapshot)
        .expect("trie-viz: failed to serialize trie snapshot to JSON");

    // Stamp the JSON directly into the template as a JS variable so we never
    // hit any URL length limits — even for a full $PATH trie.
    let html = HTML_TEMPLATE.replace(
        "const __TRIE_DATA__ = null;",
        &format!("const __TRIE_DATA__ = {};", json),
    );

    let out_path = std::env::temp_dir().join("trie-viz.html");
    std::fs::write(&out_path, html).expect("trie-viz: failed to write playground HTML to temp dir");

    eprintln!(
        "\n🌳 Trie Playground — open in your browser:\n  file://{}\n",
        out_path.display()
    );
}
