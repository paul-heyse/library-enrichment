# `datafusion_common::hash_utils::with_hashes_with_hasher`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.with_hashes_with_hasher.json).

<a id="op-b13ab121aa72a877a71d131c"></a>
## with_hashes_with_hasher

`function` · `datafusion_common::hash_utils::with_hashes_with_hasher` · datafusion-common 55.1.0

```rust
fn with_hashes_with_hasher<I, T, F, R, S>(arrays: I, hash_builder: &S, callback: F) -> error::Result<R> where I: IntoIterator<Item = T>, T: AsDynArray, F: FnOnce(&[u64]) -> error::Result<R>, S: BuildHasher
```

Source: `src/hash_utils.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates hashes for the given arrays using a thread-local buffer and a custom
hash builder, then calls the provided callback with the computed hashes.

Hash compatibility with [`with_hashes`](../operations/datafusion_common.hash_utils.with_hashes.md#op-31c70923996226d4d1940a34) follows the rules documented on
[`create_hashes_with_hasher`](../operations/datafusion_common.hash_utils.create_hashes_with_hasher.md#op-ddf297367e2d568016506a65).
