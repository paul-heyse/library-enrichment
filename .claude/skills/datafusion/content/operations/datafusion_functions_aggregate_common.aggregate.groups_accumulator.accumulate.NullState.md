# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.NullState.json).

<a id="op-b73a261d6f69ba856a29c6fc"></a>
## NullState

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState` · datafusion-functions-aggregate-common 55.1.0

```rust
struct NullState
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Track the accumulator null state per row: if any values for that
group were null and if any values have been seen at all for that group.

This is part of the inner loop for many [`GroupsAccumulator`]s,
and thus the performance is critical and so there are multiple
specialized implementations, invoked depending on the specific
combinations of the input.

Typically there are 4 potential combinations of inputs must be
special cased for performance:

* With / Without filter
* With / Without nulls in the input

If the input has nulls, then the accumulator must potentially
handle each input null value specially (e.g. for `SUM` to mark the
corresponding sum as null)

If there are filters present, `NullState` tracks if it has seen
*any* value for that group (as some values may be filtered
out). Without a filter, the accumulator is only passed groups that
had at least one value to accumulate so they do not need to track
if they have seen values for a particular group.

[`GroupsAccumulator`]: datafusion_expr_common::groups_accumulator::GroupsAccumulator

<a id="op-9334fdfb496e866061d3afe4"></a>
## accumulate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::accumulate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn accumulate<T, F>(&mut self, group_indices: &[usize], values: &PrimitiveArray<T>, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, T::Native) + Send
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [331, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Invokes `value_fn(group_index, value)` for each non null, non
filtered value of `value`, while tracking which groups have
seen null inputs and which groups have seen any inputs if necessary
# Arguments:

* `values`: the input arguments to the accumulator
* `group_indices`:  To which groups do the rows in `values` belong, (aka group_index)
* `opt_filter`: if present, only rows for which is Some(true) are included
* `value_fn`: function invoked for  (group_index, value) where value is non null

See [`accumulate`](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate.md#op-1cadf90a39061182dd59f15e), for more details on how value_fn is called

When value_fn is called it also sets

1. `self.seen_values[group_index]` to true for all rows that had a non null value

<a id="op-2466c6a86fa78d49d515c4fb"></a>
## accumulate_boolean

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::accumulate_boolean` · datafusion-functions-aggregate-common 55.1.0

```rust
fn accumulate_boolean<F>(&mut self, group_indices: &[usize], values: &BooleanArray, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F) where F: FnMut(usize, bool) + Send
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [331, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Invokes `value_fn(group_index, value)` for each non null, non
filtered value in `values`, while tracking which groups have
seen null inputs and which groups have seen any inputs, for
[`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505)s.

Since `BooleanArray` is not a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) it must be
handled specially.

See [`Self::accumulate`](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.NullState.md#op-9334fdfb496e866061d3afe4), which handles `PrimitiveArray`s, for
more details on other arguments.

<a id="op-1cafaa193c1d8aa6817c0f1b"></a>
## build

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::build` · datafusion-functions-aggregate-common 55.1.0

```rust
fn build(&mut self, emit_to: EmitTo) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [331, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Creates the a [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) representing which group_indices
should have null values (because they never saw any values)
for the `emit_to` rows.

resets the internal state appropriately

<a id="op-1aab8466e6f95c3a66625512"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [131, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0db338a48ec628a0f618616f"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 10], "end": [113, 15], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40d009c5bb1d7dedbbfb8b24"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [331, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a80cf7e744192b2a0386b2c9"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState", "path": "NullState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [331, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

return the size of all buffers allocated by this null state, not including self
