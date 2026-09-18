# `datafusion_physical_plan::joins::JoinOnRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.JoinOnRef.json).

<a id="op-9caee051dcced75013e7136f"></a>
## JoinOnRef

`type_alias` · `datafusion_physical_plan::joins::JoinOnRef` · datafusion-physical-plan 55.1.0

```rust
type JoinOnRef<'a> = &'a [(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)]
```

Source: `src/joins/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reference for JoinOn.
