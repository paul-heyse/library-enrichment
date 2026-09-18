# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls_dyn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.set_nulls_dyn.json).

<a id="op-fd98d4cc397fad1099aa7860"></a>
## set_nulls_dyn

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls_dyn` · datafusion-functions-aggregate-common 55.1.0

```rust
fn set_nulls_dyn(input: &dyn Array, nulls: Option<arrow::buffer::NullBuffer>) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/aggregate/groups_accumulator/nulls.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Replaces the nulls in the input array with the given `NullBuffer`

TODO: replace when upstreamed in arrow-rs: <https://github.com/apache/arrow-rs/issues/6528>
