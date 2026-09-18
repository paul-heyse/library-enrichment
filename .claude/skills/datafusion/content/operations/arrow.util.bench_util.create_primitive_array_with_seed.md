# `arrow::util::bench_util::create_primitive_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_array_with_seed.json).

<a id="op-7c2d859054b46703379c10ca"></a>
## create_primitive_array_with_seed

`function` · `arrow::util::bench_util::create_primitive_array_with_seed` · arrow 59.3.0

```rust
fn create_primitive_array_with_seed<T>(size: usize, null_density: f32, seed: u64) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Source: `src/util/bench_util.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of a given `size` and `null_density`
filling it with random numbers generated using the provided `seed`.
