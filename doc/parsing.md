# Parsing

## General approach

1. Lexer / tokenizer - convert raw input into tokens like `Open`, `Symbol`, `List`, `Closed`, etc
2. Parser - convert tokens into AST
3. Interpreter - converts AST into execution nodes
4. Executor - executes nodes, handles output
