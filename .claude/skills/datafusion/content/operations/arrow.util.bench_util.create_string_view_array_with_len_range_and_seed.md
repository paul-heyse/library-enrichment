# `arrow::util::bench_util::create_string_view_array_with_len_range_and_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_view_array_with_len_range_and_seed.json).

<a id="op-74e9d6fc5a2015ebad093d83"></a>
## create_string_view_array_with_len_range_and_seed

`function` · `arrow::util::bench_util::create_string_view_array_with_len_range_and_seed` · arrow 59.3.0

```rust
fn create_string_view_array_with_len_range_and_seed(size: usize, null_density: f32, range: std::ops::Range<usize>, seed: u64) -> StringViewArray
```

Source: `src/util/bench_util.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a string view array of a given range, null density and length

Arguments:
- `size`: number of  string view array
- `null_density`: density of nulls in the string view array
- `range`: range size of each string in the string view array
- `seed`: seed for the random number generator
