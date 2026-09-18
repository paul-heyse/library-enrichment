# `datafusion_optimizer::utils::evaluates_to_null`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.utils.evaluates_to_null.json).

<a id="op-60685a8d4e9d71fcab920dd2"></a>
## evaluates_to_null

`function` · `datafusion_optimizer::utils::evaluates_to_null` · datafusion-optimizer 55.1.0

```rust
fn evaluates_to_null<'a>(predicate: datafusion_expr::Expr, null_columns: impl IntoIterator<Item = &'a datafusion_common::Column>) -> datafusion_common::Result<bool>
```

Source: `src/utils.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Determines if an expression will always evaluate to null.
`c0 + 8` return true
`c0 IS NULL` return false
`CASE WHEN c0 > 1 then 0 else 1` return false
