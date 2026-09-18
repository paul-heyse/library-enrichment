# `datafusion_optimizer::push_down_filter::replace_cols_by_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.push_down_filter.replace_cols_by_name.json).

<a id="op-f31f9392089f04583897d530"></a>
## replace_cols_by_name

`function` · `datafusion_optimizer::push_down_filter::replace_cols_by_name` · datafusion-optimizer 55.1.0

```rust
fn replace_cols_by_name(e: datafusion_expr::Expr, replace_map: &std::collections::HashMap<String, impl AsRef<datafusion_expr::Expr>>) -> datafusion_common::Result<datafusion_expr::Expr>
```

Source: `src/push_down_filter.rs:1375`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

replaces columns by its name on the projection.
