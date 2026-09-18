# `datafusion_physical_optimizer::projection_pushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.projection_pushdown.json).

<a id="op-2395db2c7ea5dcb5430798ed"></a>
## projection_pushdown

`module` · `datafusion_physical_optimizer::projection_pushdown` · datafusion-physical-optimizer 55.1.0

```rust
mod projection_pushdown
```

Source: `src/projection_pushdown.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This file implements the `ProjectionPushdown` physical optimization rule.
The function [`remove_unnecessary_projections`](../operations/datafusion_physical_plan.projection.remove_unnecessary_projections.md#op-3a15e48a4a38032bc9b53811) tries to push down all
projections one by one if the operator below is amenable to this. If a
projection reaches a source, it can even disappear from the plan entirely.
