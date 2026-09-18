# `datafusion_physical_plan::projection::try_pushdown_through_join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.try_pushdown_through_join.json).

<a id="op-baf8c8d2543156a15d204a98"></a>
## try_pushdown_through_join

`function` · `datafusion_physical_plan::projection::try_pushdown_through_join` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_through_join(projection: &ProjectionExec, join_left: &std::sync::Arc<dyn ExecutionPlan>, join_right: &std::sync::Arc<dyn ExecutionPlan>, join_on: joins::utils::JoinOnRef<'_>, schema: &arrow::datatypes::SchemaRef, filter: Option<&joins::utils::JoinFilter>) -> datafusion_common::Result<Option<JoinData>>
```

Source: `src/projection.rs:815`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
