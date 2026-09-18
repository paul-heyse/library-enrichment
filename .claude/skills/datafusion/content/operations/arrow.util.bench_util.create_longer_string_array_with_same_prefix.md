# `arrow::util::bench_util::create_longer_string_array_with_same_prefix`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_longer_string_array_with_same_prefix.json).

<a id="op-65045786f75d6e92a95e5c6f"></a>
## create_longer_string_array_with_same_prefix

`function` · `arrow::util::bench_util::create_longer_string_array_with_same_prefix` · arrow 59.3.0

```rust
fn create_longer_string_array_with_same_prefix<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericStringArray<Offset>
```

Source: `src/util/bench_util.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates longer string array with same prefix, the prefix should be larger than 4 bytes,
and the string length should be larger than 12 bytes
so that we can compare the performance with StringViewArray, because StringViewArray has 4 bytes inline for view
