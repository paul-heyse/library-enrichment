# `datafusion_expr::utils::collect_subquery_cols`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.collect_subquery_cols.json).

<a id="op-2e8518ac32c66f63f679d270"></a>
## collect_subquery_cols

`function` · `datafusion_expr::utils::collect_subquery_cols` · datafusion-expr 55.1.0

```rust
fn collect_subquery_cols(exprs: &[Expr], subquery_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<std::collections::BTreeSet<datafusion_common::Column>>
```

Source: `src/utils.rs:1423`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determine the set of [`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6)s produced by the subquery.
