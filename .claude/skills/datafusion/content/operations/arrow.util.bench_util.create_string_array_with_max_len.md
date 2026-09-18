# `arrow::util::bench_util::create_string_array_with_max_len`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_array_with_max_len.json).

<a id="op-bc060ba0077c5d19a04f5a3d"></a>
## create_string_array_with_max_len

`function` · `arrow::util::bench_util::create_string_array_with_max_len` · arrow 59.3.0

```rust
fn create_string_array_with_max_len<Offset: OffsetSizeTrait>(size: usize, null_density: f32, max_str_len: usize) -> GenericStringArray<Offset>
```

Source: `src/util/bench_util.rs:329`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length
