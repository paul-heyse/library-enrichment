# `arrow::util::bench_util::create_string_view_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_view_array.json).

<a id="op-e64ba72fd75692567c575004"></a>
## create_string_view_array

`function` · `arrow::util::bench_util::create_string_view_array` · arrow 59.3.0

```rust
fn create_string_view_array(size: usize, null_density: f32) -> StringViewArray
```

Source: `src/util/bench_util.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) string view array of a given size and null density.

See `create_string_array` above for more details.
