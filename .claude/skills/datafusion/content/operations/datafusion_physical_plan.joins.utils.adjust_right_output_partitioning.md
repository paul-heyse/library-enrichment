# `datafusion_physical_plan::joins::utils::adjust_right_output_partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.adjust_right_output_partitioning.json).

<a id="op-16a78c4fe5a9fa8795c614fa"></a>
## adjust_right_output_partitioning

`function` · `datafusion_physical_plan::joins::utils::adjust_right_output_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn adjust_right_output_partitioning(right_partitioning: &Partitioning, left_columns_len: usize) -> datafusion_common::Result<Partitioning>
```

Source: `src/joins/utils.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Adjust the right out partitioning to new Column Index
