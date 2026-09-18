# `datafusion_physical_plan::check_if_same_properties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.check_if_same_properties.json).

<a id="op-ac447d6557424dbc67cd8618"></a>
## check_if_same_properties

`macro` · `datafusion_physical_plan::check_if_same_properties` · datafusion-physical-plan 55.1.0

```rust
macro_rules! check_if_same_properties
```

Source: `src/execution_plan.rs:2015`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Helper macro to avoid properties re-computation if passed children properties
the same as plan already has. Could be used to implement fast-path for method
[`ExecutionPlan::with_new_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-c94174b2f94c0eb5bf9111f2).

New call sites should route through [`replace_children_if_necessary`](../operations/datafusion_physical_plan.execution_plan.replace_children_if_necessary.md#op-39321a5fcf4c72a03a82abd3),
which applies this check together with the child-pointer short-circuit
(see [`replace_children_if_necessary`](../operations/datafusion_physical_plan.execution_plan.replace_children_if_necessary.md#op-39321a5fcf4c72a03a82abd3) for the layered policy). This
macro remains for direct-caller sites that have not been migrated yet.
