# `datafusion_common::partitioning::validate_range_split_points`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.partitioning.validate_range_split_points.json).

<a id="op-71391c69f3417fdb3eb71084"></a>
## validate_range_split_points

`function` · `datafusion_common::partitioning::validate_range_split_points` · datafusion-common 55.1.0

```rust
fn validate_range_split_points(split_points: &[SplitPoint], sort_options: &[arrow::compute::SortOptions]) -> Result<()>
```

Source: `src/partitioning.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Validates that split points match the ordering width and are strictly
ordered according to the provided sort options.
