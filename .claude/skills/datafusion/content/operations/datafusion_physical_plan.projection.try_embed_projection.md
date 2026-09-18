# `datafusion_physical_plan::projection::try_embed_projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.try_embed_projection.json).

<a id="op-e289a4a8d849312623510d8f"></a>
## try_embed_projection

`function` · `datafusion_physical_plan::projection::try_embed_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_embed_projection<Exec: EmbeddedProjection + 'static>(projection: &ProjectionExec, execution_plan: &Exec) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>>
```

Source: `src/projection.rs:736`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Some projection can't be pushed down left input or right input of hash join because filter or on need may need some columns that won't be used in later.
By embed those projection to hash join, we can reduce the cost of build_batch_from_indices in hash join (build_batch_from_indices need to can compute::take() for each column) and avoid unnecessary output creation.
