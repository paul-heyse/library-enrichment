# `datafusion_physical_plan::visitor::accept`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.visitor.accept.json).

<a id="op-43874cdc8b2745b3e61053de"></a>
## accept

`function` · `datafusion_physical_plan::visitor::accept` · datafusion-physical-plan 55.1.0

```rust
fn accept<V: ExecutionPlanVisitor>(plan: &dyn ExecutionPlan, visitor: &mut V) -> Result<(), V::Error>
```

Source: `src/visitor.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Visit all children of this plan, according to the order defined on `ExecutionPlanVisitor`.
