# `arrow::util::bench_util::create_string_view_array_with_max_len`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_view_array_with_max_len.json).

<a id="op-522a579fca7d7c9370c53b28"></a>
## create_string_view_array_with_max_len

`function` · `arrow::util::bench_util::create_string_view_array_with_max_len` · arrow 59.3.0

```rust
fn create_string_view_array_with_max_len(size: usize, null_density: f32, max_str_len: usize) -> StringViewArray
```

Source: `src/util/bench_util.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length
