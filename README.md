# Nash - Nelson Again Shell

Nash is a shell with a [lisp-inspired syntax](doc/Syntax.md), written in Rust. It is incomplete and not entirely in a working state as I have dropped it for other interests. The basic building blocks are

- State-machine tokenizer and recursive descent parser in [nash-parser/](nash-parser/)
- Line editor in [nash-line/](nash-line/)
  - Tab-completion library code in [nash-line/src/autocomplete/](nash-line/src/autocomplete/)
- Evaluator / runner in [nash-core/](nash-core/)

You can actually try it if you'd like:
```bash
cargo run -p nash-exe
```
```lisp
(echo "hello from Nash")
(echo (printf nested))
```

The runtime does support external commands, quoted arguments, multiple top-level forms, and nested command execution. I never got to bindings, pipelines, redirection, logical operators, and conditionals. The tab-completion library code is written but wiring it in is a `todo!()` so if you press tab it will panic...

Honorable mention: I vibe coded a developer tool to help me visualize the autocomplete in [trie-viz/](trie-viz/)

## A note on AI usage
This was my first Rust project so AI code generation was minimal. I did use it extensively to ask questions and seek clarification. And to vibe code adjacent developer tooling.