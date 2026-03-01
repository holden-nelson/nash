use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub terminals: Vec<String>,
    pub terminals_in_subtrie: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            terminals: Vec::new(),
            terminals_in_subtrie: 0,
        }
    }

    fn completes_token(&self) -> bool {
        !self.terminals.is_empty()
    }
}

pub struct Trie {
    pub root_node: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root_node: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, s: &str) {
        let mut current_node = &mut self.root_node;

        for c in s.chars() {
            current_node.terminals_in_subtrie += 1;

            let c_norm = norm(c);

            current_node = current_node
                .children
                .entry(c_norm)
                .or_insert_with(TrieNode::new);
        }

        current_node.terminals_in_subtrie += 1;
        current_node.terminals.push(s.to_owned());
    }

    pub fn get_completions(&self, s: &str) -> Option<Vec<String>> {
        let prefix_node = self.find_node(s);

        if let Some(node) = prefix_node {
            let mut completions = Vec::new();
            let mut prefix = s.to_owned();

            Self::dfs(node, &mut prefix, &mut completions);

            Some(completions)
        } else {
            None
        }
    }

    fn find_node(&self, s: &str) -> Option<&TrieNode> {
        let mut current_node = &self.root_node;

        for c in s.chars() {
            let c_norm = norm(c);

            if let Some(child_node) = current_node.children.get(&c_norm) {
                current_node = &child_node;
            } else {
                return None;
            }
        }

        Some(current_node)
    }

    fn dfs(node: &TrieNode, prefix: &mut String, completions: &mut Vec<String>) {
        if node.completes_token() {
            completions.extend(node.terminals.iter().cloned());
        }

        for (c, child_node) in &node.children {
            prefix.push(*c);
            Self::dfs(child_node, prefix, completions);
            prefix.pop();
        }
    }
}

fn norm(c: char) -> char {
    c.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trie_with(words: &[&str]) -> Trie {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }

    fn sorted(v: Option<Vec<String>>) -> Option<Vec<String>> {
        v.map(|mut s| {
            s.sort();
            s
        })
    }

    // ── insert / basic retrieval ─────────────────────────────────────────────

    #[test]
    fn returns_none_for_unknown_prefix() {
        let trie = trie_with(&["hello", "world"]);
        assert_eq!(trie.get_completions(&mut "xyz".to_owned()), None);
    }

    #[test]
    fn returns_exact_match() {
        let trie = trie_with(&["hello"]);
        assert_eq!(
            sorted(trie.get_completions(&mut "hello".to_owned())),
            Some(vec!["hello".to_owned()])
        );
    }

    #[test]
    fn returns_all_completions_for_shared_prefix() {
        let trie = trie_with(&["cat", "car", "card", "care", "dog"]);
        assert_eq!(
            sorted(trie.get_completions(&mut "car".to_owned())),
            Some(vec!["car".to_owned(), "card".to_owned(), "care".to_owned()])
        );
    }

    #[test]
    fn empty_prefix_returns_all_words() {
        let words = ["alpha", "beta", "gamma"];
        let trie = trie_with(&words);
        let mut expected: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        expected.sort();
        assert_eq!(
            sorted(trie.get_completions(&mut "".to_owned())),
            Some(expected)
        );
    }

    // ── case insensitivity ───────────────────────────────────────────────────

    #[test]
    fn uppercase_prefix_matches_lowercase_insertions() {
        let trie = trie_with(&["git", "grep"]);
        assert_eq!(
            sorted(trie.get_completions(&mut "G".to_owned())),
            Some(vec!["git".to_owned(), "grep".to_owned()])
        );
    }

    #[test]
    fn lowercase_prefix_matches_uppercase_insertions() {
        let trie = trie_with(&["GIT", "GREP"]);
        assert_eq!(
            sorted(trie.get_completions(&mut "g".to_owned())),
            Some(vec!["GIT".to_owned(), "GREP".to_owned()])
        );
    }

    #[test]
    fn completions_preserve_original_casing() {
        let trie = trie_with(&["CamelCase", "camelback"]);
        let result = sorted(trie.get_completions(&mut "camel".to_owned()));
        assert_eq!(
            result,
            Some(vec!["CamelCase".to_owned(), "camelback".to_owned()])
        );
    }

    // ── terminals_in_subtrie ─────────────────────────────────────────────────

    #[test]
    fn terminals_in_subtrie_counts_correctly() {
        let trie = trie_with(&["foo", "foobar", "foobaz"]);
        // root should account for all 3 words
        assert_eq!(trie.root_node.terminals_in_subtrie, 3);
    }

    // ── duplicates ───────────────────────────────────────────────────────────

    #[test]
    fn inserting_same_word_twice_returns_it_twice() {
        let trie = trie_with(&["echo", "echo"]);
        let result = trie.get_completions(&mut "echo".to_owned()).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|s| s == "echo"));
    }
}
