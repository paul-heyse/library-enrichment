# `sqlparser::ast::comments::Comment::SingleLine`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.comments.Comment.SingleLine.json).

<a id="op-867e71347354860e998db7ef"></a>
## content

`struct_field` · `sqlparser::ast::comments::Comment::SingleLine::content` · sqlparser 0.62.0

```rust
content: String
```

Source: `src/ast/comments.rs:183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The content of the comment (including trailing newline, if any).

<a id="op-e69e72a81b5e4f7be4b004fb"></a>
## prefix

`struct_field` · `sqlparser::ast::comments::Comment::SingleLine::prefix` · sqlparser 0.62.0

```rust
prefix: String
```

Source: `src/ast/comments.rs:185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The prefix introducing the comment (e.g. `--`, `#`).
