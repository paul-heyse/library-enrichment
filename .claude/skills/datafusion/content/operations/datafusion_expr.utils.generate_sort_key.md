# `datafusion_expr::utils::generate_sort_key`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.generate_sort_key.json).

<a id="op-c7aeff9adec7c07cc18682b7"></a>
## generate_sort_key

`function` · `datafusion_expr::utils::generate_sort_key` · datafusion-expr 55.1.0

```rust
fn generate_sort_key(partition_by: &[Expr], order_by: &[expr::Sort]) -> datafusion_common::Result<Vec<(expr::Sort, bool)>>
```

Source: `src/utils.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Generate a sort key for a given window expr's partition_by and order_by expr
