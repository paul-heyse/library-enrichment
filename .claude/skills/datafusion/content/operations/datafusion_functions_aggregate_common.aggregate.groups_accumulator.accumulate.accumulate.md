# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate.json).

<a id="op-1cadf90a39061182dd59f15e"></a>
## accumulate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn accumulate<T, F>(group_indices: &[usize], values: &arrow::array::PrimitiveArray<T>, opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, T::Native) + Send
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Invokes `value_fn(group_index, value)` for each non null, non
filtered value of `value`,

# Arguments:

* `group_indices`:  To which groups do the rows in `values` belong, (aka group_index)
* `values`: the input arguments to the accumulator
* `opt_filter`: if present, only rows for which is Some(true) are included
* `value_fn`: function invoked for  (group_index, value) where value is non null

# Example

```text
 ┌─────────┐   ┌─────────┐   ┌ ─ ─ ─ ─ ┐
 │ ┌─────┐ │   │ ┌─────┐ │     ┌─────┐
 │ │  2  │ │   │ │ 200 │ │   │ │  t  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  2  │ │   │ │ 100 │ │   │ │  f  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  0  │ │   │ │ 200 │ │   │ │  t  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  1  │ │   │ │ 200 │ │   │ │NULL │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  0  │ │   │ │ 300 │ │   │ │  t  │ │
 │ └─────┘ │   │ └─────┘ │     └─────┘
 └─────────┘   └─────────┘   └ ─ ─ ─ ─ ┘

group_indices   values        opt_filter
```

In the example above, `value_fn` is invoked for each (group_index,
value) pair where `opt_filter[i]` is true and values is non null

```text
value_fn(2, 200)
value_fn(0, 200)
value_fn(0, 300)
```
