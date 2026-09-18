# `datafusion_expr::utils::compare_sort_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.compare_sort_expr.json).

<a id="op-97755250821baa0f37c5d059"></a>
## compare_sort_expr

`function` · `datafusion_expr::utils::compare_sort_expr` · datafusion-expr 55.1.0

```rust
fn compare_sort_expr(sort_expr_a: &expr::Sort, sort_expr_b: &expr::Sort, schema: &datafusion_common::DFSchemaRef) -> std::cmp::Ordering
```

Source: `src/utils.rs:563`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Compare the sort expr as PostgreSQL's common_prefix_cmp():
<https://github.com/postgres/postgres/blob/master/src/backend/optimizer/plan/planner.c>
