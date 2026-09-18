# `arrow::util::bench_util::create_f32_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_f32_array.json).

<a id="op-f9405a291fd35174c3a94fe2"></a>
## create_f32_array

`function` · `arrow::util::bench_util::create_f32_array` · arrow 59.3.0

```rust
fn create_f32_array(size: usize, nan_density: f32) -> Float32Array
```

Source: `src/util/bench_util.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) f32 array of a given size and nan-value density
