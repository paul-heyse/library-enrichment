# `sqlparser::ast::value::escape_quoted_string`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.escape_quoted_string.json).

<a id="op-aec83e601ea3872af7d5d50d"></a>
## escape_quoted_string

`function` · `sqlparser::ast::value::escape_quoted_string` · sqlparser 0.62.0

```rust
fn escape_quoted_string(string: &str, quote: char) -> EscapeQuotedString<'_>
```

Source: `src/ast/value.rs:598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return a helper which formats `string` for inclusion inside a quoted
literal that uses `quote` as the delimiter.
