# `datafusion_physical_plan::filter::PhysicalExprPairRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.PhysicalExprPairRef.json).

<a id="op-d570e9a35266349e1aa8952c"></a>
## PhysicalExprPairRef

`type_alias` · `datafusion_physical_plan::filter::PhysicalExprPairRef` · datafusion-physical-plan 55.1.0

```rust
type PhysicalExprPairRef<'a> = (&'a std::sync::Arc<dyn PhysicalExpr>, &'a std::sync::Arc<dyn PhysicalExpr>)
```

Source: `src/filter.rs:1404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pair of `Arc<dyn PhysicalExpr>`s
