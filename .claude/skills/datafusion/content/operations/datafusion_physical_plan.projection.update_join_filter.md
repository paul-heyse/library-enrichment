# `datafusion_physical_plan::projection::update_join_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.update_join_filter.json).

<a id="op-1ec63a416380956dabf7c681"></a>
## update_join_filter

`function` · `datafusion_physical_plan::projection::update_join_filter` · datafusion-physical-plan 55.1.0

```rust
fn update_join_filter(projection_left_exprs: &[(super::expressions::Column, String)], projection_right_exprs: &[(super::expressions::Column, String)], join_filter: &joins::utils::JoinFilter, left_field_size: usize) -> Option<joins::utils::JoinFilter>
```

Source: `src/projection.rs:1262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to update the column indices of a [`JoinFilter`](../operations/datafusion_physical_plan.joins.join_filter.JoinFilter.md#op-2c36e92fdae023ebe60d1dc7) as if the input of
the join was replaced by a projection.
