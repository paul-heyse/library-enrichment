# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate`

Crate `datafusion-functions-aggregate-common` · 5 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.json`](../model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.json)

## SeenValues

`enum` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues`

```rust
enum SeenValues
```

**Variants**: `All`, `Some`

**Derives**: Debug, Default

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.SeenValues.md).


If the input has nulls, then the accumulator must potentially
handle each input null value specially (e.g. for `SUM` to mark the
corresponding sum as null)

If there are filters present, `NullState` tracks if it has seen
*any* value for that group (as some values may be filtered
out). Without a filter, the accumulator is only passed groups that
had at least one value to accumulate so they do not need to track
if they have seen values for a particular group.

---

## accumulate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate`

```rust
fn accumulate<T, F>(group_indices: &[usize], values: &arrow::array::PrimitiveArray<T>, opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, T::Native) + Send
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate.md).


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

---

## accumulate_indices

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_indices`

```rust
fn accumulate_indices<F>(group_indices: &[usize], nulls: Option<&arrow::buffer::NullBuffer>, opt_filter: Option<&arrow::array::BooleanArray>, index_fn: F) where F: FnMut(usize) + Send
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate_indices.md).


This function is called to update the accumulator state per row
when the value is not needed (e.g. COUNT)

`F`: Invoked like `value_fn(group_index) for all non null values
passing the filter. Note that no tracking is done for null inputs
or which groups have seen any values

See [`NullState::accumulate`], for more details on other
arguments.

---

## accumulate_multiple

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_multiple`

```rust
fn accumulate_multiple<T, F>(group_indices: &[usize], value_columns: &[&arrow::array::PrimitiveArray<T>], opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, usize, &[&arrow::array::PrimitiveArray<T>]) + Send
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate_multiple.md).


Accumulates with multiple accumulate(value) columns. (e.g. `corr(c1, c2)`)

This method assumes that for any input record index, if any of the value column
is null, or it's filtered out by `opt_filter`, then the record would be ignored.
(Won't be accumulated by `value_fn`)

# Arguments

* `group_indices` - To which groups do the rows in `value_columns` belong
* `value_columns` - The input arrays to accumulate
* `opt_filter` - Optional filter array. If present, only rows where filter is `Some(true)` are included
* `value_fn` - Callback function for each valid row, with parameters:
    * `group_idx`: The group index for the current row
    * `batch_idx`: The index of the current row in the input arrays
    * `columns`: Reference to all input arrays for accessing values

---

## NullState

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::NullState`

Also reachable as `datafusion::physical_expr::NullState`, `datafusion_physical_expr::NullState`

```rust
struct NullState
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn accumulate<T, F>(&mut self, group_indices: &[usize], values: &PrimitiveArray<T>, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, T::Native) + Send
fn accumulate_boolean<F>(&mut self, group_indices: &[usize], values: &BooleanArray, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F) where F: FnMut(usize, bool) + Send
fn build(&mut self, emit_to: EmitTo) -> Option<NullBuffer>
fn new() -> Self
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.NullState.md).


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

---
