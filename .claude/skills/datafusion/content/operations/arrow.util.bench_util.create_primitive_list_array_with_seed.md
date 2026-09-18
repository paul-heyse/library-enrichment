# `arrow::util::bench_util::create_primitive_list_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_list_array_with_seed.json).

<a id="op-923739107a6b736c5fc61101"></a>
## create_primitive_list_array_with_seed

`function` · `arrow::util::bench_util::create_primitive_list_array_with_seed` · arrow 59.3.0

```rust
fn create_primitive_list_array_with_seed<O, T>(size: usize, null_density: f32, list_null_density: f32, max_list_size: usize, seed: u64) -> GenericListArray<O> where O: OffsetSizeTrait, T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Source: `src/util/bench_util.rs:486`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a List/LargeList Array  of primitive values

Arguments:
- `size`: number of lists in the array
- `null_density`: density of nulls in the list array
- `list_null_density`: density of nulls in the primitive arrays inside the lists
- `max_list_size`: maximum size of each list (actual size is random between 0 and max_list_size)
- `seed`: seed for the random number generator
