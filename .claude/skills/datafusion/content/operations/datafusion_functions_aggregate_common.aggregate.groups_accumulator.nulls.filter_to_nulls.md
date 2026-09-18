# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filter_to_nulls`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.filter_to_nulls.json).

<a id="op-b478980437e42afda8004955"></a>
## filter_to_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::filter_to_nulls` · datafusion-functions-aggregate-common 55.1.0

```rust
fn filter_to_nulls(filter: &arrow::array::BooleanArray) -> arrow::buffer::NullBuffer
```

Source: `src/aggregate/groups_accumulator/nulls.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Converts an aggregate filter expression to a `NullBuffer`.

The `NullBuffer` is
* `true` (representing valid) for filter values that were `Some(true)`
* `false` (representing null) for filter values that were `Some(false)` or `None`
