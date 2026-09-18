# `arrow::util::bench_util::create_primitive_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_array.json).

<a id="op-e695a8faac6a2d3e63d797b7"></a>
## create_primitive_array

`function` · `arrow::util::bench_util::create_primitive_array` · arrow 59.3.0

```rust
fn create_primitive_array<T>(size: usize, null_density: f32) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Source: `src/util/bench_util.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates an random (but fixed-seeded) array of a given size and null density
