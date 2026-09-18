# `datafusion_physical_plan::projection::update_ordering_requirement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.update_ordering_requirement.json).

<a id="op-f2ddd45b2fbed610b9ee30f8"></a>
## update_ordering_requirement

`function` · `datafusion_physical_plan::projection::update_ordering_requirement` · datafusion-physical-plan 55.1.0

```rust
fn update_ordering_requirement(reqs: datafusion_physical_expr_common::sort_expr::LexRequirement, projected_exprs: &[ProjectionExpr]) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexRequirement>>
```

Source: `src/projection.rs:1088`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Updates the given lexicographic requirement according to given projected
expressions using the [`update_expr`](../operations/datafusion_physical_expr.projection.update_expr.md#op-5318705f25c637abc4758b4e) function.
