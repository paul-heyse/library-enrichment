# `arrow::util::bench_util::create_primitive_list_view_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_list_view_array.json).

<a id="op-9edbd75a5ca054e42c351a6b"></a>
## create_primitive_list_view_array

`function` · `arrow::util::bench_util::create_primitive_list_view_array` · arrow 59.3.0

```rust
fn create_primitive_list_view_array<O, T>(size: usize, null_density: f32, list_null_density: f32, max_list_size: usize) -> GenericListViewArray<O> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>, O: OffsetSizeTrait
```

Source: `src/util/bench_util.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a ListViewArray of primitive values using a fixed seed

See [`create_primitive_list_array_with_seed`](../operations/arrow.util.bench_util.create_primitive_list_array_with_seed.md#op-923739107a6b736c5fc61101) for details on arguments.
