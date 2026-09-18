# `arrow::util::bench_util::create_f64_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_f64_array_with_seed.json).

<a id="op-3b90e1bb45b9fd5dd8314589"></a>
## create_f64_array_with_seed

`function` · `arrow::util::bench_util::create_f64_array_with_seed` · arrow 59.3.0

```rust
fn create_f64_array_with_seed(size: usize, nan_density: f32, seed: u64) -> Float64Array
```

Source: `src/util/bench_util.rs:863`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random f64 array of a given size and nan-value density based on a given seed
