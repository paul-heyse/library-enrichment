# `sqlparser::tokenizer::Whitespace::SingleLineComment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Whitespace.SingleLineComment.json).

<a id="op-5486874d9678a03e38fd807b"></a>
## comment

`struct_field` · `sqlparser::tokenizer::Whitespace::SingleLineComment::comment` · sqlparser 0.62.0

```rust
comment: String
```

Source: `src/tokenizer.rs:509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The content of the comment (without the prefix).

<a id="op-65d2a195f709b36a2b321da5"></a>
## prefix

`struct_field` · `sqlparser::tokenizer::Whitespace::SingleLineComment::prefix` · sqlparser 0.62.0

```rust
prefix: String
```

Source: `src/tokenizer.rs:511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The prefix used for the comment (for example `--` or `#`).
