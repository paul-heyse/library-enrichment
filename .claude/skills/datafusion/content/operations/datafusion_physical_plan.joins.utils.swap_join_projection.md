# `datafusion_physical_plan::joins::utils::swap_join_projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.swap_join_projection.json).

<a id="op-fa8f5cf53cca3ea19d3d3a50"></a>
## swap_join_projection

`function` · `datafusion_physical_plan::joins::utils::swap_join_projection` · datafusion-physical-plan 55.1.0

```rust
fn swap_join_projection(left_schema_len: usize, right_schema_len: usize, projection: Option<&[usize]>, join_type: &datafusion_common::JoinType) -> Option<Vec<usize>>
```

Source: `src/joins/utils.rs:2081`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This function swaps the given join's projection.
