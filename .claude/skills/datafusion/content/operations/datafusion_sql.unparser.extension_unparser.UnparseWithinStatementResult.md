# `datafusion_sql::unparser::extension_unparser::UnparseWithinStatementResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.extension_unparser.UnparseWithinStatementResult.json).

<a id="op-b685039c3158038f6c388e16"></a>
## UnparseWithinStatementResult

`enum` · `datafusion_sql::unparser::extension_unparser::UnparseWithinStatementResult` · datafusion-sql 55.1.0

```rust
enum UnparseWithinStatementResult
```

Source: `src/unparser/extension_unparser.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The result of unparsing a custom logical node within a statement.

<a id="op-15c5cfbc862b34b2acc8d55f"></a>
## Modified

`variant` · `datafusion_sql::unparser::extension_unparser::UnparseWithinStatementResult::Modified` · datafusion-sql 55.1.0

```rust
Modified
```

Source: `src/unparser/extension_unparser.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

If the custom logical node was successfully unparsed within a statement.

<a id="op-13f14243cf65157c6984a576"></a>
## Unmodified

`variant` · `datafusion_sql::unparser::extension_unparser::UnparseWithinStatementResult::Unmodified` · datafusion-sql 55.1.0

```rust
Unmodified
```

Source: `src/unparser/extension_unparser.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

If the custom logical node wasn't unparsed.
