# `sqlparser::tokenizer`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.json).

<a id="op-b42ac5030a9bf88910f04191"></a>
## tokenizer

`module` · `sqlparser::tokenizer` · sqlparser 0.62.0

```rust
mod tokenizer
```

Source: `src/tokenizer.rs:18`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Tokenizer

The tokenizer (a.k.a. lexer) converts a string into a sequence of tokens.

The tokens then form the input for the parser, which outputs an Abstract Syntax Tree (AST).
