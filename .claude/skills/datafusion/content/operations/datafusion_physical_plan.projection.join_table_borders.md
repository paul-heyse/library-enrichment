# `datafusion_physical_plan::projection::join_table_borders`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.join_table_borders.json).

<a id="op-9694f62298e71189f8ed812f"></a>
## join_table_borders

`function` · `datafusion_physical_plan::projection::join_table_borders` · datafusion-physical-plan 55.1.0

```rust
fn join_table_borders(left_table_column_count: usize, projection_as_columns: &[(super::expressions::Column, String)]) -> (i32, i32)
```

Source: `src/projection.rs:1213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the last index before encountering a column coming from the right table when traveling
through the projection from left to right, and the last index before encountering a column
coming from the left table when traveling through the projection from right to left.
If there is no column in the projection coming from the left side, it returns (-1, ...),
if there is no column in the projection coming from the right side, it returns (..., projection length).
