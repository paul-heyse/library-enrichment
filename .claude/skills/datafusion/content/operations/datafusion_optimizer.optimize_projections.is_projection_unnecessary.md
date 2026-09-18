# `datafusion_optimizer::optimize_projections::is_projection_unnecessary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimize_projections.is_projection_unnecessary.json).

<a id="op-0df19c3735b744ed503d0f8e"></a>
## is_projection_unnecessary

`function` · `datafusion_optimizer::optimize_projections::is_projection_unnecessary` · datafusion-optimizer 55.1.0

```rust
fn is_projection_unnecessary(input: &datafusion_expr::logical_plan::LogicalPlan, proj_exprs: &[datafusion_expr::Expr]) -> datafusion_common::Result<bool>
```

Source: `src/optimize_projections/mod.rs:869`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Projection is unnecessary, when
- input schema of the projection, output schema of the projection are same, and
- all projection expressions are either Column or Literal
