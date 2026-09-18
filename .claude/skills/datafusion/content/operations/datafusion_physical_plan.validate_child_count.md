# `datafusion_physical_plan::validate_child_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.validate_child_count.json).

<a id="op-f72813713d94a140a0135070"></a>
## validate_child_count

`macro` · `datafusion_physical_plan::validate_child_count` · datafusion-physical-plan 55.1.0

```rust
macro_rules! validate_child_count
```

Source: `src/execution_plan.rs:2033`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Helper macro to validate that replacement children match a plan's existing
child count.

This is useful for [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) implementations that
need to preserve the same child-count validation behavior.
