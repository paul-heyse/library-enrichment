# `arrow::util::bench_util::create_fsb_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_fsb_array.json).

<a id="op-64ec168b8e625c0858b2af90"></a>
## create_fsb_array

`function` · `arrow::util::bench_util::create_fsb_array` · arrow 59.3.0

```rust
fn create_fsb_array(size: usize, null_density: f32, value_len: usize) -> FixedSizeBinaryArray
```

Source: `src/util/bench_util.rs:742`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates an random (but fixed-seeded) array of a given size and null density
