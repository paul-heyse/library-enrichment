# `datafusion_physical_expr::physical_expr::create_physical_partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.create_physical_partitioning.json).

<a id="op-1918bdce2dcaa71e9f56b067"></a>
## create_physical_partitioning

`function` · `datafusion_physical_expr::physical_expr::create_physical_partitioning` · datafusion-physical-expr 55.1.0

```rust
fn create_physical_partitioning(partitioning: &datafusion_expr::Partitioning, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<Partitioning>
```

Source: `src/physical_expr.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create physical partitioning from logical partitioning.

See [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for details on the `planning_ctx` argument.
