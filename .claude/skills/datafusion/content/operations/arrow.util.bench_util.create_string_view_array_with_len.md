# `arrow::util::bench_util::create_string_view_array_with_len`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_view_array_with_len.json).

<a id="op-16ced8da08f03ac925461092"></a>
## create_string_view_array_with_len

`function` · `arrow::util::bench_util::create_string_view_array_with_len` · arrow 59.3.0

```rust
fn create_string_view_array_with_len(size: usize, null_density: f32, str_len: usize, mixed: bool) -> StringViewArray
```

Source: `src/util/bench_util.rs:419`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of a given size, null density and length
