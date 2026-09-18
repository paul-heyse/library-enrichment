# `arrow::util::bench_util::create_month_day_nano_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_month_day_nano_array_with_seed.json).

<a id="op-d9d7b72b697e4762c508c292"></a>
## create_month_day_nano_array_with_seed

`function` · `arrow::util::bench_util::create_month_day_nano_array_with_seed` · arrow 59.3.0

```rust
fn create_month_day_nano_array_with_seed(size: usize, null_density: f32, seed: u64) -> IntervalMonthDayNanoArray
```

Source: `src/util/bench_util.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of a given `size` and `null_density`
filling it with random [`IntervalMonthDayNano`](../operations/arrow_buffer.interval.IntervalMonthDayNano.md#op-124a76e3b87e96892e283f4a) generated using the provided `seed`.
