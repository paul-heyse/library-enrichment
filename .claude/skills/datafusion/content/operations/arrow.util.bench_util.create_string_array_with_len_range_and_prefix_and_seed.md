# `arrow::util::bench_util::create_string_array_with_len_range_and_prefix_and_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_array_with_len_range_and_prefix_and_seed.json).

<a id="op-ba4ebaf52e9ba89143a855ef"></a>
## create_string_array_with_len_range_and_prefix_and_seed

`function` · `arrow::util::bench_util::create_string_array_with_len_range_and_prefix_and_seed` · arrow 59.3.0

```rust
fn create_string_array_with_len_range_and_prefix_and_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, min_str_len: usize, max_str_len: usize, prefix: &str, seed: u64) -> GenericStringArray<Offset>
```

Source: `src/util/bench_util.rs:222`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) of a given `size` and `null_density`
filling it with random strings with lengths in the specified range,
all starting with the provided `prefix`, generated using the provided `seed`.
