# `datafusion_physical_plan::projection::update_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.update_ordering.json).

<a id="op-ef279399c4b4f86e647fd5df"></a>
## update_ordering

`function` · `datafusion_physical_plan::projection::update_ordering` · datafusion-physical-plan 55.1.0

```rust
fn update_ordering(ordering: datafusion_physical_expr_common::sort_expr::LexOrdering, projected_exprs: &[ProjectionExpr]) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexOrdering>>
```

Source: `src/projection.rs:1070`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Updates the given lexicographic ordering according to given projected
expressions using the [`update_expr`](../operations/datafusion_physical_expr.projection.update_expr.md#op-5318705f25c637abc4758b4e) function.
