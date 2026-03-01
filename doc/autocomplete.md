# Autocomplete plan

- Define some trait `Completer` or something with a function `complete()`.
  - `complete()` mutably borrows the buffer and manipulates it as it would like
- When Line Editor is instantiated, my Completer is provided

Completer:

- Would really like case-insensitive completions
- Start with just installed executables in Path
- On `new()`, walks directories in path and gathers all executables. Puts them in trie
- On tab, we need to lex and parse the input
  - I have the existing lexer and parser, but it errors out when you give it input that is incomplete.
  - Makes me wonder if we need to separate the logic out a touch so we can adapt it for both use cases
- If we determine that the "current" thing being completed on is an installed executable - search the trie for it
- If we find a completion, replace the current word in the buffer with the correctly-cased word found in trie

Case-insensitive completions:

- My nodes need to be keyed by a normalized (lowercase) char, but for actual completions and LCP we need to keep some casing information around
- The node should look like this

```rs
pub struct TrieNode {
    children: HashMap<char, TrieNode>, // normalized edges
    terminals: Vec<String>,            // if this node ends any tokens
    lcp_under: Option<String>,         // case-sensitive LCP of terminals below
    count_under: usize,                // number of terminals below
}
```

- As we insert executables, we calculate the lcp of the executable we're inserting and the existing prefix on that node and cache it on the node
  - So we're walking the trie, inserting `fooBar`
  - We make it to a node `f` -> `o`
  - This node has `lcp_under` -> `Some("foobaz")`
  - we replace with `lcp("foobaz", "fooBar")` -> `Some("foo")`
    - not `Some("fooba")` cause differing casing of `b`
  - we also increment `count_under` cause we know this thing is gonna terminate down there somewhere
