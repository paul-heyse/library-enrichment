# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate_indices.json).

<a id="op-e562208a0a15d813be8e06d4"></a>
## accumulate_indices

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_indices` · datafusion-functions-aggregate-common 55.1.0

```rust
fn accumulate_indices<F>(group_indices: &[usize], nulls: Option<&arrow::buffer::NullBuffer>, opt_filter: Option<&arrow::array::BooleanArray>, index_fn: F) where F: FnMut(usize) + Send
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:542`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

This function is called to update the accumulator state per row
when the value is not needed (e.g. COUNT)

`F`: Invoked like `value_fn(group_index) for all non null values
passing the filter. Note that no tracking is done for null inputs
or which groups have seen any values

See [`NullState::accumulate`](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.NullState.md#op-9334fdfb496e866061d3afe4), for more details on other
arguments.
