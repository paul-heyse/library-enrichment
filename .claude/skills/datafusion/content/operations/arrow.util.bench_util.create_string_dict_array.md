# `arrow::util::bench_util::create_string_dict_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_dict_array.json).

<a id="op-4f56e87a9a356f76ec40542d"></a>
## create_string_dict_array

`function` · `arrow::util::bench_util::create_string_dict_array` · arrow 59.3.0

```rust
fn create_string_dict_array<K: ArrowDictionaryKeyType>(size: usize, null_density: f32, str_len: usize) -> DictionaryArray<K>
```

Source: `src/util/bench_util.rs:456`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates an random (but fixed-seeded) array of a given size and null density
consisting of random 4 character alphanumeric strings
