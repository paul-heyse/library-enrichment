# `arrow::util::bench_util::create_string_array_with_len`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_array_with_len.json).

<a id="op-1575897e6a5ec9f132a8a515"></a>
## create_string_array_with_len

`function` · `arrow::util::bench_util::create_string_array_with_len` · arrow 59.3.0

```rust
fn create_string_array_with_len<Offset: OffsetSizeTrait>(size: usize, null_density: f32, str_len: usize) -> GenericStringArray<Offset>
```

Source: `src/util/bench_util.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of a given size, null density and length
