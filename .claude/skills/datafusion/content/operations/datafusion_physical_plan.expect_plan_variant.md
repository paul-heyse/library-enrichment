# `datafusion_physical_plan::expect_plan_variant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.expect_plan_variant.json).

<a id="op-d3d8cdd90bdf2ba51f2afd07"></a>
## expect_plan_variant

`macro` · `datafusion_physical_plan::expect_plan_variant` · datafusion-physical-plan 55.1.0

```rust
macro_rules! expect_plan_variant
```

Source: `src/proto.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Assert that a [`PhysicalPlanNode`](../operations/datafusion_proto_models.generated.datafusion.PhysicalPlanNode.md#op-5e215028df0efda21d0133c8) carries the expected `PhysicalPlanType`
variant, returning a reference to the inner payload, else an `internal_err!`.
Mirrors `expect_expr_variant!` on the expression side. Field access on the
result auto-derefs through the `Box` that boxed variants use.
