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
    terminals_in_subtrie: usize,       // number of terminals below
}
```

- As we insert executables, we increment the cached number of terminals in the subtrie so we know how many on a quick glance

Completion behavior:

- If there is no autocompletion or LCP for what the user has entered, ring bell
- If there is exactly one autocompletion, complete it
- If there is more than one autocompletion,
  - and there are less than 37 autocompletions
    - display autocompletions
      - and let the user tab through them
  - and there are more than 36 autocompletions
    - ask if user wants to display
      - if yes
        - display them
          - and let the user tab through them
