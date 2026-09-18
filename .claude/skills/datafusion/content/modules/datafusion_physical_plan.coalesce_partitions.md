# `datafusion_physical_plan::coalesce_partitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coalesce_partitions.json).

<a id="op-be7022af2994f3918aa471e1"></a>
## coalesce_partitions

`module` · `datafusion_physical_plan::coalesce_partitions` · datafusion-physical-plan 55.1.0

```rust
mod coalesce_partitions
```

Source: `src/coalesce_partitions.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Defines the merge plan for executing partitions in parallel and then merging the results
into a single partition
