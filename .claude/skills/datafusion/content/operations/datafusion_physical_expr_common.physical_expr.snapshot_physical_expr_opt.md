# `datafusion_physical_expr_common::physical_expr::snapshot_physical_expr_opt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.snapshot_physical_expr_opt.json).

<a id="op-10930068c0622236b2eb4c6f"></a>
## snapshot_physical_expr_opt

`function` · `datafusion_physical_expr_common::physical_expr::snapshot_physical_expr_opt` · datafusion-physical-expr-common 55.1.0

```rust
fn snapshot_physical_expr_opt(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Source: `src/physical_expr.rs:962`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Take a snapshot of the given `PhysicalExpr` if it is dynamic.

Take a snapshot of this `PhysicalExpr` if it is dynamic.
This is used to capture the current state of `PhysicalExpr`s that may contain
dynamic references to other operators in order to serialize it over the wire
or treat it via downcast matching.

See the documentation of [`PhysicalExpr::snapshot`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-eac5395e633cbe5d577748bf) for more details.

# Returns

Returns a `[`Transformed`] indicating whether a snapshot was taken,
along with the resulting `PhysicalExpr`.
