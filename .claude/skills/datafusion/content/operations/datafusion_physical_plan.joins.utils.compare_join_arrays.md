# `datafusion_physical_plan::joins::utils::compare_join_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.compare_join_arrays.json).

<a id="op-5ece6addc543cf9c29f3ae0a"></a>
## compare_join_arrays

`function` · `datafusion_physical_plan::joins::utils::compare_join_arrays` · datafusion-physical-plan 55.1.0

```rust
fn compare_join_arrays(left_arrays: &[arrow::array::ArrayRef], left: usize, right_arrays: &[arrow::array::ArrayRef], right: usize, sort_options: &[arrow_schema::SortOptions], null_equality: datafusion_common::NullEquality) -> datafusion_common::Result<std::cmp::Ordering>
```

Source: `src/joins/utils.rs:2452`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get comparison result of two rows of join arrays
