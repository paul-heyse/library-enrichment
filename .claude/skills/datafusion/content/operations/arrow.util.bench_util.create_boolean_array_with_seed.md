# `arrow::util::bench_util::create_boolean_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_boolean_array_with_seed.json).

<a id="op-6364d35fb38c91877ae51d39"></a>
## create_boolean_array_with_seed

`function` · `arrow::util::bench_util::create_boolean_array_with_seed` · arrow 59.3.0

```rust
fn create_boolean_array_with_seed(size: usize, null_density: f32, true_density: f32, seed: u64) -> BooleanArray where rand::distr::StandardUniform: Distribution<bool>
```

Source: `src/util/bench_util.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random array of a given size and null density based on the provided seed
