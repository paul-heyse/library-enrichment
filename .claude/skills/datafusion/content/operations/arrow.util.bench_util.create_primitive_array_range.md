# `arrow::util::bench_util::create_primitive_array_range`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_array_range.json).

<a id="op-dc366c22b8943c32c80202fe"></a>
## create_primitive_array_range

`function` · `arrow::util::bench_util::create_primitive_array_range` · arrow 59.3.0

```rust
fn create_primitive_array_range<T>(size: usize, null_density: f32, value_range: std::ops::Range<T::Native>) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>, T::Native: SampleUniform
```

Source: `src/util/bench_util.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates an random (but fixed-seeded) array of a given size and null density,
all the values located in the given range
