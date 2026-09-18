# `datafusion_common::hash_utils::HashState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.HashState.json).

<a id="op-1daf08f037f63bf69ba920d2"></a>
## HashState

`trait` · `datafusion_common::hash_utils::HashState` · datafusion-common 55.1.0

```rust
trait HashState: BuildHasher
```

Source: `src/hash_utils.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Hash state used by [`create_hashes`](../operations/datafusion_common.hash_utils.create_hashes.md#op-8f6ed733a45cc2f583a1ae0d).

Multi-column hashing folds the previous column hash into a fresh hasher
before hashing the next column. This trait keeps that seeded hasher in the
same foldhash tier as the top-level hash state.

<a id="op-2c9b594e42147cdb9323a6b0"></a>
## SeededState

`assoc_type` · `datafusion_common::hash_utils::HashState::SeededState` · datafusion-common 55.1.0

```rust
SeededState
```

Source: `src/hash_utils.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26486126400ed936770c61e0"></a>
## seeded_state

`function` · `datafusion_common::hash_utils::HashState::seeded_state` · datafusion-common 55.1.0

```rust
fn seeded_state(&self, seed: u64) -> Self::SeededState
```

Source: `src/hash_utils.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
