# `datafusion_physical_plan::joins::JoinOn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.JoinOn.json).

<a id="op-57cf29afb533af650be5455d"></a>
## JoinOn

`type_alias` · `datafusion_physical_plan::joins::JoinOn` · datafusion-physical-plan 55.1.0

```rust
type JoinOn = Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>
```

Source: `src/joins/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The on clause of the join, as vector of (left, right) columns.
