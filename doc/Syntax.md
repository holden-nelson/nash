# Nash Shell Language Syntax

The Nash shell language is a simple lisp-like language. Some example commands:

```
(ls -la ~)

; or |
(pipe (cat file.txt)
      (grep error)
      (sort))

; redirection - or use >
( out (cat file.txt)
      output.txt)

; two way redirection
(append (in cat input.txt) output.txt)
```

## Basic features

- Bind symbols to values with `def`
- Piping with `pipe` or the `|` shorthand
- Redirection with `in`, `out`, and `append`, or the equivalent bash shorthands
- Logical operators `and`, `or`, and `not`
- Conditional branching with `if`
- Comments with ;
- Quote literals with `"` and `'`
- Escaping with \
- Identifier fallback - unbound symbols fallback to strings

Advanced features under consideration:

- Macros
- Functional builtins like `compose` or `map`
- Data structures (lists, maps, sets, ...?)
- Error handling (`try-catch` or `with-error`)
- Concurrency primitives
- Metaprogramming

## Escaping and quotes

The backslash `\` character is used for escaping.

**Symbols (unquoted)**

- `\ `: escape a space
- `\(` or `)`: escape a paren
- `\"` or `\'`: escape a quote
- `\\t` or `\\n` or any other unicode whitespace: escape whitespace

**Single Quotes** and **Double Quotes**
Single quotes and double quotes function identically. They allow you to

- type spaces without escaping
- skip binding evaluation

So, within quotes:

- `\"` escaped a double quote
- `\'` escapes a single quote

Escaping quotes within quotes may not strictly be necessary - 'hell"o' would evaluate to `hell"o` - but you can always escape either type from within either type.

For the binding evaluation opt-out, in this example

```
(def ls 'ls -la`)
(ls ~)
```

would list all files in long form, because `ls` has been bound to `ls -la`. But

```
(def ls 'ls -la`)
('ls' ~)
```

would not, because, although `ls` is bound, `ls` skips the binding evaluation.
