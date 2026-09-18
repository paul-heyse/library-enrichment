# `arrow::util::bench_util::create_string_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_array.json).

<a id="op-a275c4cfd73aa6bdb86b7948"></a>
## create_string_array

`function` · `arrow::util::bench_util::create_string_array` · arrow 59.3.0

```rust
fn create_string_array<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericStringArray<Offset>
```

Source: `src/util/bench_util.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) string array of a given size and null density.

Strings have a random length
between 0 and 400 alphanumeric characters. `0..400` is chosen to cover a wide range of common string lengths,
which have a dramatic impact on performance of some queries, e.g. LIKE/ILIKE/regex.
