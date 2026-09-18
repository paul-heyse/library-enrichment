# `datafusion_physical_plan::projection::EmbeddedProjection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.EmbeddedProjection.json).

<a id="op-c49ddea72adeda9a0fcb3644"></a>
## EmbeddedProjection

`trait` · `datafusion_physical_plan::projection::EmbeddedProjection` · datafusion-physical-plan 55.1.0

```rust
trait EmbeddedProjection: ExecutionPlan + Sized
```

Source: `src/projection.rs:730`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Trait for execution plans that can embed a projection, avoiding a separate
[`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158) wrapper.

# Empty projections

`Some(vec![])` is a valid projection that produces zero output columns while
preserving the correct row count. Implementors must ensure that runtime batch
construction still returns batches with the right number of rows even when no
columns are selected (e.g. for `SELECT count(1) … JOIN …`).

<a id="op-b819d002e85b070a4e0d0929"></a>
## with_projection

`function` · `datafusion_physical_plan::projection::EmbeddedProjection::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Source: `src/projection.rs:731`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
