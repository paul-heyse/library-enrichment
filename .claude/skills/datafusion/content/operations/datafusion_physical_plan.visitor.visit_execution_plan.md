# `datafusion_physical_plan::visitor::visit_execution_plan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.visitor.visit_execution_plan.json).

<a id="op-9cf68bd819895986658a994a"></a>
## visit_execution_plan

`function` · `datafusion_physical_plan::visitor::visit_execution_plan` · datafusion-physical-plan 55.1.0

```rust
fn visit_execution_plan<V: ExecutionPlanVisitor>(plan: &dyn ExecutionPlan, visitor: &mut V) -> Result<(), V::Error>
```

Source: `src/visitor.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Recursively calls `pre_visit` and `post_visit` for this node and
all of its children, as described on [`ExecutionPlanVisitor`](../operations/datafusion_physical_plan.visitor.ExecutionPlanVisitor.md#op-2b3e60aa91348196cf730331)
