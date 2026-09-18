# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::apply_filter_as_nulls`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.apply_filter_as_nulls.json).

<a id="op-f4f8a35edac4d4252475f89c"></a>
## apply_filter_as_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::apply_filter_as_nulls` · datafusion-functions-aggregate-common 55.1.0

```rust
fn apply_filter_as_nulls(input: &dyn Array, opt_filter: Option<&arrow::array::BooleanArray>) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/aggregate/groups_accumulator/nulls.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Applies optional filter to input, returning a new array of the same type
with the same data, but with any values that were filtered out set to null
