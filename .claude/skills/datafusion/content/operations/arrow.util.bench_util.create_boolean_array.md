# `arrow::util::bench_util::create_boolean_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_boolean_array.json).

<a id="op-b6e86e78667c695816537620"></a>
## create_boolean_array

`function` · `arrow::util::bench_util::create_boolean_array` · arrow 59.3.0

```rust
fn create_boolean_array(size: usize, null_density: f32, true_density: f32) -> BooleanArray where rand::distr::StandardUniform: Distribution<bool>
```

Source: `src/util/bench_util.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) array of a given size and null density
