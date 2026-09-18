# `datafusion_datasource::morsel::Morselizer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.Morselizer.json).

<a id="op-4dc8c6c12cccc0db0eec6703"></a>
## Morselizer

`trait` · `datafusion_datasource::morsel::Morselizer` · datafusion-datasource 55.1.0

```rust
trait Morselizer: Send + Sync + Debug
```

Source: `src/morsel/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A Morselizer takes a single [`PartitionedFile`](../operations/datafusion_datasource.PartitionedFile.md#op-25225d18e868fe5ac8eeb098) and creates the initial planner
for that file.

This is the entry point for morsel driven I/O.

<a id="op-d19604025be92fd7ffcdf0fc"></a>
## plan_file

`function` · `datafusion_datasource::morsel::Morselizer::plan_file` · datafusion-datasource 55.1.0

```rust
fn plan_file(&self, file: PartitionedFile) -> Result<Box<dyn MorselPlanner>>
```

Source: `src/morsel/mod.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the initial [`MorselPlanner`](../operations/datafusion_datasource.morsel.MorselPlanner.md#op-75e3192888d61e52d84762e9) for this file.

Morselizing a file may involve CPU work, such as parsing parquet
metadata and evaluating pruning predicates. It should NOT do any I/O
work, such as reading from the file. Any needed I/O should be done using
[`MorselPlan::with_pending_planner`](../operations/datafusion_datasource.morsel.MorselPlan.md#op-8ebbaa1d07bbe282c53bae7c).
