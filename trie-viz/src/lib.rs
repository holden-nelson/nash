use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use nash_line::autocomplete::{Trie, TrieNode};
use serde::Serialize;

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
    /// Longest common prefix of all strings reachable from this node.
    lcp_of_subtrie: Option<String>,
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
        lcp_of_subtrie: node.lcp_of_subtrie.clone(),
        terminals_in_subtrie: node.terminals_in_subtrie,
        children,
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Serialise `trie` to JSON, base64-encode it, and print a `file://` URL to
/// `stderr` that opens the co-located `index.html` playground page.
///
/// Usage:
/// ```rust,ignore
/// trie_viz::log_playground_url(&my_trie);
/// ```
pub fn log_playground_url(trie: &Trie) {
    let snapshot = snapshot_node(&trie.root_node, None);

    let json = serde_json::to_string(&snapshot)
        .expect("trie-viz: failed to serialize trie snapshot to JSON");

    let encoded = URL_SAFE_NO_PAD.encode(json.as_bytes());

    // CARGO_MANIFEST_DIR is resolved at compile time to the absolute path of
    // the trie-viz crate directory, giving us a stable path to index.html.
    let html_path = concat!(env!("CARGO_MANIFEST_DIR"), "/index.html");

    eprintln!(
        "\n🌳 Trie Playground — open in your browser:\n  file://{}#data={}\n",
        html_path, encoded
    );
}
