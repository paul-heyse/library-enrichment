# `datafusion_optimizer::utils::is_restrict_null_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.utils.is_restrict_null_predicate.json).

<a id="op-b1b0e2286cbe54bcae55337a"></a>
## is_restrict_null_predicate

`function` · `datafusion_optimizer::utils::is_restrict_null_predicate` · datafusion-optimizer 55.1.0

```rust
fn is_restrict_null_predicate<'a>(predicate: datafusion_expr::Expr, join_cols_of_predicate: impl IntoIterator<Item = &'a datafusion_common::Column>) -> datafusion_common::Result<bool>
```

Source: `src/utils.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Determine whether a predicate can restrict NULLs. e.g.
`c0 > 8` return true;
`c0 IS NULL` return false.
