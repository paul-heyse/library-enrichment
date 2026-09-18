# `arrow::util::bench_util::create_binary_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_binary_array.json).

<a id="op-69c0fd1336b1f8936e8617b4"></a>
## create_binary_array

`function` · `arrow::util::bench_util::create_binary_array` · arrow 59.3.0

```rust
fn create_binary_array<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericBinaryArray<Offset>
```

Source: `src/util/bench_util.rs:663`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates an random (but fixed-seeded) binary array of a given size and null density
