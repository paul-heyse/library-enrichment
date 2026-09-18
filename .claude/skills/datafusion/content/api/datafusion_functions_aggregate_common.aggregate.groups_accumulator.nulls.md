# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls`

Crate `datafusion-functions-aggregate-common` · 5 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.json`](../model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.json)

## apply_filter_as_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::apply_filter_as_nulls`

```rust
fn apply_filter_as_nulls(input: &dyn Array, opt_filter: Option<&arrow::array::BooleanArray>) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.apply_filter_as_nulls.md).


Applies optional filter to input, returning a new array of the same type
with the same data, but with any values that were filtered out set to null

---

## filter_to_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filter_to_nulls`

```rust
fn filter_to_nulls(filter: &arrow::array::BooleanArray) -> arrow::buffer::NullBuffer
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.filter_to_nulls.md).


Converts an aggregate filter expression to a `NullBuffer`.

The `NullBuffer` is
* `true` (representing valid) for filter values that were `Some(true)`
* `false` (representing null) for filter values that were `Some(false)` or `None`

---

## filtered_null_mask

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filtered_null_mask`

```rust
fn filtered_null_mask(opt_filter: Option<&arrow::array::BooleanArray>, input: &dyn Array) -> Option<arrow::buffer::NullBuffer>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.filtered_null_mask.md).


Compute an output validity mask for an array that has been filtered

This can be used to compute nulls for the output of
[`GroupsAccumulator::convert_to_state`], which quickly applies an optional
filter to the input rows by setting any filtered rows to NULL in the output.
Subsequent applications of  aggregate functions that ignore NULLs (most of
them) will thus ignore the filtered rows as well.

# Output element is `true` (and thus output is non-null)

A `true` in the output represents non null output for all values that were *both*:

* `true` in any `opt_filter` (aka values that passed the filter)

* `non null` in `input`

# Output element is `false` (and thus output is null)

A `false` in the output represents an input that was *either*:

* `null`

* filtered (aka the value was `false` or `null` in the filter)

# Example

```text
┌─────┐           ┌─────┐            ┌─────┐
│true │           │NULL │            │false│
│true │    │      │true │            │true │
│true │ ───┼───   │false│  ────────▶ │false│       filtered_nulls
│false│    │      │NULL │            │false│
│false│           │true │            │false│
└─────┘           └─────┘            └─────┘
array           opt_filter           output
 .nulls()

false = NULL    true  = pass          false = NULL       Meanings
true  = valid   false = filter        true  = valid
                NULL  = filter
```

[`GroupsAccumulator::convert_to_state`]: datafusion_expr_common::groups_accumulator::GroupsAccumulator

---

## set_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls`

```rust
fn set_nulls<T: ArrowNumericType + Send>(array: arrow::array::PrimitiveArray<T>, nulls: Option<arrow::buffer::NullBuffer>) -> arrow::array::PrimitiveArray<T>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.set_nulls.md).


Sets the validity mask for a `PrimitiveArray` to `nulls`
replacing any existing null mask

See [`set_nulls_dyn`] for a version that works with `Array`

---

## set_nulls_dyn

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls_dyn`

```rust
fn set_nulls_dyn(input: &dyn Array, nulls: Option<arrow::buffer::NullBuffer>) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.set_nulls_dyn.md).


Replaces the nulls in the input array with the given `NullBuffer`

TODO: replace when upstreamed in arrow-rs: <https://github.com/apache/arrow-rs/issues/6528>

---
