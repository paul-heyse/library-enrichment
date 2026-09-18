# `datafusion_common::hash_utils::create_hashes_with_hasher`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.create_hashes_with_hasher.json).

<a id="op-ddf297367e2d568016506a65"></a>
## create_hashes_with_hasher

`function` · `datafusion_common::hash_utils::create_hashes_with_hasher` · datafusion-common 55.1.0

```rust
fn create_hashes_with_hasher<'a, I, T, S>(arrays: I, hash_builder: &S, hashes_buffer: &'a mut [u64]) -> error::Result<&'a mut [u64]> where I: IntoIterator<Item = T>, T: AsDynArray, S: BuildHasher
```

Source: `src/hash_utils.rs:1269`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates hash values for every row using a caller-provided hash builder.

The number of rows to hash is determined by `hashes_buffer.len()`.
`hashes_buffer` should be pre-sized appropriately.

# Hash compatibility

Hash values are not guaranteed to be bit-for-bit identical to those from
[`create_hashes`](../operations/datafusion_common.hash_utils.create_hashes.md#op-8f6ed733a45cc2f583a1ae0d), even when `hash_builder` also implements [`HashState`](../operations/datafusion_common.hash_utils.HashState.md#op-1daf08f037f63bf69ba920d2).
The optimized [`HashState`](../operations/datafusion_common.hash_utils.HashState.md#op-1daf08f037f63bf69ba920d2) path seeds the hasher from the previous hash
when rehashing some primitive and byte-view values, whereas this function
combines independently computed hashes. Use one API consistently if hashes
are persisted or exchanged.
