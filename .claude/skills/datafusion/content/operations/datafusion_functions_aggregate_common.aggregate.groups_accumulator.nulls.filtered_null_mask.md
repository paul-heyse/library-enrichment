# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filtered_null_mask`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.filtered_null_mask.json).

<a id="op-823294a0ee2c29e8350e54ac"></a>
## filtered_null_mask

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filtered_null_mask` · datafusion-functions-aggregate-common 55.1.0

```rust
fn filtered_null_mask(opt_filter: Option<&arrow::array::BooleanArray>, input: &dyn Array) -> Option<arrow::buffer::NullBuffer>
```

Source: `src/aggregate/groups_accumulator/nulls.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

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
