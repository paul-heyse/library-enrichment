# `datafusion_physical_plan::projection::try_pushdown_through_join_with_column_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.try_pushdown_through_join_with_column_indices.json).

<a id="op-acdad99d2dd1b2ed1b0ad886"></a>
## try_pushdown_through_join_with_column_indices

`function` · `datafusion_physical_plan::projection::try_pushdown_through_join_with_column_indices` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_through_join_with_column_indices(projection: &ProjectionExec, join_left: &std::sync::Arc<dyn ExecutionPlan>, join_right: &std::sync::Arc<dyn ExecutionPlan>, join_on: joins::utils::JoinOnRef<'_>, schema: &arrow::datatypes::SchemaRef, filter: Option<&joins::utils::JoinFilter>, column_indices: &[joins::utils::ColumnIndex]) -> datafusion_common::Result<Option<JoinData>>
```

Source: `src/projection.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Attempts to move a projection below a join by mapping each join output
column to the child column that produced it.

`schema` is the complete output schema of the join, not either child's
schema. `column_indices` must contain one entry for each field in `schema`.
Each [`JoinSide::Left`](../operations/datafusion_common.join_type.JoinSide.md#op-5f7f4c68abf9e75f1138f7b4) or [`JoinSide::Right`](../operations/datafusion_common.join_type.JoinSide.md#op-fb7b485de584896a40f58901) entry identifies the source
child and uses an index relative to that child's schema.

[`JoinSide::None`](../operations/datafusion_common.join_type.JoinSide.md#op-5e0a7412df9ff9b25212aa0e) identifies a column produced by the join itself, such as
a mark column. If `projection` references such a column, this function
returns `Ok(None)` because neither child can produce it.

Returns `Ok(None)` when the projection cannot be pushed down safely.

# Errors

Returns an error if `column_indices` does not match `schema` or contains an
index outside the corresponding child schema.
