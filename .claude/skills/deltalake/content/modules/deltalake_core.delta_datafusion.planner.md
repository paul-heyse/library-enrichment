# `deltalake_core::delta_datafusion::planner`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.planner.json).

<a id="op-d22211841d4fff4241c186c8"></a>
## planner

`module` · `deltalake_core::delta_datafusion::planner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod planner
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L1).

Source: `crates/core/src/delta_datafusion/planner.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Custom planners for datafusion so that you can convert custom nodes, can be used
to trace custom metrics in an operation

# Example

#[derive(Clone)]
struct MergeMetricExtensionPlanner {}

#[macro@async_trait]
impl ExtensionPlanner for MergeMetricExtensionPlanner {
    async fn plan_extension(
        &self,
        planner: &dyn PhysicalPlanner,
        node: &dyn UserDefinedLogicalNode,
        _logical_inputs: &[&LogicalPlan],
        physical_inputs: &[Arc<dyn ExecutionPlan>],
        session_state: &SessionState,
    ) -> DataFusionResult<Option<Arc<dyn ExecutionPlan>>> {}

let merge_planner = DeltaPlanner::<MergeMetricExtensionPlanner> {
    extension_planner: MergeMetricExtensionPlanner {}
};

let state = state.with_query_planner(Arc::new(merge_planner));

Unresolved upstream links (retained, not inferred): `macro@async_trait`, `Arc<dyn ExecutionPlan>`.
