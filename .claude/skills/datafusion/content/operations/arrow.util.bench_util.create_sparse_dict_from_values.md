# `arrow::util::bench_util::create_sparse_dict_from_values`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_sparse_dict_from_values.json).

<a id="op-5ced4a3618100c5ea4ea802d"></a>
## create_sparse_dict_from_values

`function` · `arrow::util::bench_util::create_sparse_dict_from_values` · arrow 59.3.0

```rust
fn create_sparse_dict_from_values<K>(size: usize, null_density: f32, values: &dyn Array, key_range: std::ops::Range<K::Native>) -> DictionaryArray<K> where K: ArrowDictionaryKeyType, rand::distr::StandardUniform: Distribution<K::Native>, K::Native: SampleUniform
```

Source: `src/util/bench_util.rs:781`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random (but fixed-seeded) dictionary array of a given size and null density
with the provided values array and key range
