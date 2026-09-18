# `arrow::util::bench_util::create_primitive_fixed_size_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_fixed_size_list_array.json).

<a id="op-5711bac99f0f79e568ac7d95"></a>
## create_primitive_fixed_size_list_array

`function` · `arrow::util::bench_util::create_primitive_fixed_size_list_array` · arrow 59.3.0

```rust
fn create_primitive_fixed_size_list_array<T>(size: usize, null_density: f32, value_null_density: f32, list_size: i32) -> FixedSizeListArray where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Source: `src/util/bench_util.rs:884`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a FixedSizeList array of primitive values

Arguments:
- `size`: number of fixed-size lists in the array
- `null_density`: density of nulls in the fixed-size list array (row-level nulls)
- `value_null_density`: density of nulls in the primitive values inside each list
- `list_size`: fixed size of each list element
