# `arrow::util::bench_util::create_longer_string_view_array_with_same_prefix`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_longer_string_view_array_with_same_prefix.json).

<a id="op-69cc8a3a21768c2b79995b06"></a>
## create_longer_string_view_array_with_same_prefix

`function` · `arrow::util::bench_util::create_longer_string_view_array_with_same_prefix` · arrow 59.3.0

```rust
fn create_longer_string_view_array_with_same_prefix(size: usize, null_density: f32) -> StringViewArray
```

Source: `src/util/bench_util.rs:195`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates longer string view array with same prefix, the prefix should be larger than 4 bytes,
and the string length should be larger than 12 bytes
so that we can compare the StringArray performance with StringViewArray, because StringViewArray has 4 bytes inline for view
