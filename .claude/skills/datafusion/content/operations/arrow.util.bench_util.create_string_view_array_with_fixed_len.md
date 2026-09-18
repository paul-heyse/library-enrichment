# `arrow::util::bench_util::create_string_view_array_with_fixed_len`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_view_array_with_fixed_len.json).

<a id="op-d31c81ad44e7c8b2a812b40f"></a>
## create_string_view_array_with_fixed_len

`function` · `arrow::util::bench_util::create_string_view_array_with_fixed_len` · arrow 59.3.0

```rust
fn create_string_view_array_with_fixed_len(size: usize, null_density: f32, str_len: usize) -> StringViewArray
```

Source: `src/util/bench_util.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of a given size, null density and length
