# `datafusion_physical_expr_common::physical_expr::snapshot_generation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.snapshot_generation.json).

<a id="op-297e75607be761bf81f57611"></a>
## snapshot_generation

`function` · `datafusion_physical_expr_common::physical_expr::snapshot_generation` · datafusion-physical-expr-common 55.1.0

```rust
fn snapshot_generation(expr: &std::sync::Arc<dyn PhysicalExpr>) -> u64
```

Source: `src/physical_expr.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Check the generation of this `PhysicalExpr`.
Dynamic `PhysicalExpr`s may have a generation that is incremented
every time the state of the `PhysicalExpr` changes.
If the generation changes that means this `PhysicalExpr` or one of its children
has changed since the last time it was evaluated.

This algorithm will not produce collisions as long as the structure of the
`PhysicalExpr` does not change and no `PhysicalExpr` decrements its own generation.
