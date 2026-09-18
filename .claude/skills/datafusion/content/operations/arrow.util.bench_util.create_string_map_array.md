# `arrow::util::bench_util::create_string_map_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_map_array.json).

<a id="op-5487ee8986560699506e253c"></a>
## create_string_map_array

`function` · `arrow::util::bench_util::create_string_map_array` · arrow 59.3.0

```rust
fn create_string_map_array<T>(size: usize, null_density: f32, max_map_size: usize, key_len: usize) -> MapArray where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Source: `src/util/bench_util.rs:922`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a Map array with string keys and primitive values

Arguments:
- `size`: number of map entries in the array
- `null_density`: density of nulls in the map array (row-level nulls)
- `max_map_size`: maximum number of key-value pairs per map entry
  (actual size is random between 0 and max_map_size)
- `key_len`: length of each random string key
