# `sqlparser::ast::value::escape_double_quote_string`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.escape_double_quote_string.json).

<a id="op-16cbf390e1cca816f7c492f5"></a>
## escape_double_quote_string

`function` · `sqlparser::ast::value::escape_double_quote_string` · sqlparser 0.62.0

```rust
fn escape_double_quote_string(s: &str) -> EscapeQuotedString<'_>
```

Source: `src/ast/value.rs:608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Convenience wrapper for escaping strings for double-quoted literals (`").`
