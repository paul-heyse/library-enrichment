# `arrow::util::bench_util::create_binary_array_with_seed`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_binary_array_with_seed.json).

<a id="op-2bfd73518275f559ee05461e"></a>
## create_binary_array_with_seed

`function` · `arrow::util::bench_util::create_binary_array_with_seed` · arrow 59.3.0

```rust
fn create_binary_array_with_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, bytes_seed: u64, bytes_length_seed: u64) -> GenericBinaryArray<Offset>
```

Source: `src/util/bench_util.rs:682`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a random [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) of a given `size` and `null_density`
filling it with random bytes, generated using the provided `seed`s.

the `bytes_seed` is used to seed the RNG for generating the byte values,
while the `bytes_length_seed` is used to seed the RNG for generating the length of an array item

These values can be the same as they are used to seed different RNGs internally.
