# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.set_nulls.json).

<a id="op-7067e0bb83fa98feb5e8c945"></a>
## set_nulls

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::nulls::set_nulls` · datafusion-functions-aggregate-common 55.1.0

```rust
fn set_nulls<T: ArrowNumericType + Send>(array: arrow::array::PrimitiveArray<T>, nulls: Option<arrow::buffer::NullBuffer>) -> arrow::array::PrimitiveArray<T>
```

Source: `src/aggregate/groups_accumulator/nulls.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Sets the validity mask for a `PrimitiveArray` to `nulls`
replacing any existing null mask

See [`set_nulls_dyn`](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.nulls.set_nulls_dyn.md#op-fd98d4cc397fad1099aa7860) for a version that works with `Array`
