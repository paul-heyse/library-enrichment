# `datafusion_sql::unparser::extension_unparser::UnparseToStatementResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.extension_unparser.UnparseToStatementResult.json).

<a id="op-86556f031a5b5720b91ccb44"></a>
## UnparseToStatementResult

`enum` · `datafusion_sql::unparser::extension_unparser::UnparseToStatementResult` · datafusion-sql 55.1.0

```rust
enum UnparseToStatementResult
```

Source: `src/unparser/extension_unparser.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The result of unparsing a custom logical node to a statement.

<a id="op-d2226a1fdd2c98032b9d7f1e"></a>
## Modified

`variant` · `datafusion_sql::unparser::extension_unparser::UnparseToStatementResult::Modified` · datafusion-sql 55.1.0

```rust
Modified
```

Source: `src/unparser/extension_unparser.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

If the custom logical node was successfully unparsed to a statement.

<a id="op-068945e97d4853a78688deb3"></a>
## Unmodified

`variant` · `datafusion_sql::unparser::extension_unparser::UnparseToStatementResult::Unmodified` · datafusion-sql 55.1.0

```rust
Unmodified
```

Source: `src/unparser/extension_unparser.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

If the custom logical node wasn't unparsed.
