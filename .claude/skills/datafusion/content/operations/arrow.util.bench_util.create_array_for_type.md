# `arrow::util::bench_util::create_array_for_type`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_array_for_type.json).

<a id="op-e9f1bd525fd9d367b725df07"></a>
## create_array_for_type

`function` · `arrow::util::bench_util::create_array_for_type` · arrow 59.3.0

```rust
fn create_array_for_type(data_type: &DataType, size: usize, null_density: f32) -> ArrayRef
```

Source: `src/util/bench_util.rs:955`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random array for the given [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c), `size`, and `null_density`.

Useful for building arrays and record batches in benchmarks without
repeating per-type construction logic. Panics on unsupported types.
