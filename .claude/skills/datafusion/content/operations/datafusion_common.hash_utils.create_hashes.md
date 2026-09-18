# `datafusion_common::hash_utils::create_hashes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.create_hashes.json).

<a id="op-8f6ed733a45cc2f583a1ae0d"></a>
## create_hashes

`function` · `datafusion_common::hash_utils::create_hashes` · datafusion-common 55.1.0

```rust
fn create_hashes<'a, I, T>(arrays: I, random_state: &impl HashState, hashes_buffer: &'a mut [u64]) -> error::Result<&'a mut [u64]> where I: IntoIterator<Item = T>, T: AsDynArray
```

Source: `src/hash_utils.rs:1239`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates hash values for every row, based on the values in the columns.

The number of rows to hash is determined by `hashes_buffer.len()`.
`hashes_buffer` should be pre-sized appropriately.
