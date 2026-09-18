# `arrow::util::bench_util::create_f16_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_f16_array.json).

<a id="op-f055e5c8f8391f8957b242db"></a>
## create_f16_array

`function` · `arrow::util::bench_util::create_f16_array` · arrow 59.3.0

```rust
fn create_f16_array(size: usize, nan_density: f32) -> Float16Array
```

Source: `src/util/bench_util.rs:818`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) f16 array of a given size and nan-value density
