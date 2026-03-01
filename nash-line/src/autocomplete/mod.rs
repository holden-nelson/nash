use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    terminals: Vec<String>,
    lcp_of_subtrie: Option<String>,
    terminals_in_subtrie: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            terminals: Vec::new(),
            lcp_of_subtrie: None,
            terminals_in_subtrie: 0,
        }
    }

    fn completes_token(&self) -> bool {
        !self.terminals.is_empty()
    }
}

pub struct Trie {
    root_node: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root_node: TrieNode::new(),
        }
    }

    fn lcp(s1: &str, s2: &str) -> String {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let n = b1.len().min(b2.len());

        let mut i = 0;
        while i < n && b1[i] == b2[i] {
            i += 1;
        }

        // we may be inside of a unicode character
        while i > 0 && !s1.is_char_boundary(i) {
            i -= 1;
        }

        s1[..i].to_owned()
    }

    fn norm(c: char) -> char {
        c.to_ascii_lowercase()
    }

    fn update_node(node: &mut TrieNode, s: &str) {
        match node.lcp_of_subtrie.as_mut() {
            None => node.lcp_of_subtrie = Some(s.to_owned()),
            Some(existing) => {
                *existing = Self::lcp(existing, s);
            }
        }

        node.terminals_in_subtrie += 1;
    }

    pub fn insert(&mut self, s: &str) {
        let mut current_node = &mut self.root_node;

        for c in s.chars() {
            Self::update_node(current_node, s);

            let c_norm = Self::norm(c);

            current_node = current_node
                .children
                .entry(c_norm)
                .or_insert_with(TrieNode::new);
        }

        Self::update_node(current_node, s);

        current_node.terminals.push(s.to_owned());
    }
}
