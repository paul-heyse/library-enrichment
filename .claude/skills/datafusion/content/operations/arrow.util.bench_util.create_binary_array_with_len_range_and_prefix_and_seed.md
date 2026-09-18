# `arrow::util::bench_util::create_binary_array_with_len_range_and_prefix_and_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_binary_array_with_len_range_and_prefix_and_seed.json).

<a id="op-b34a099ab97176958b4983ee"></a>
## create_binary_array_with_len_range_and_prefix_and_seed

`function` · `arrow::util::bench_util::create_binary_array_with_len_range_and_prefix_and_seed` · arrow 59.3.0

```rust
fn create_binary_array_with_len_range_and_prefix_and_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, min_len: usize, max_len: usize, prefix: &[u8], seed: u64) -> GenericBinaryArray<Offset>
```

Source: `src/util/bench_util.rs:710`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) of a given `size` and `null_density`
filling it with random bytes with lengths in the specified range,
all starting with the provided `prefix`, generated using the provided `seed`.

