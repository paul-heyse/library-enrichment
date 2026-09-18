# `datafusion_common::hash_utils::HLL_RANDOM_STATE`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.HLL_RANDOM_STATE.json).

<a id="op-a5c9d37843237807ccf17578"></a>
## HLL_RANDOM_STATE

`constant` · `datafusion_common::hash_utils::HLL_RANDOM_STATE` · datafusion-common 55.1.0

```rust
const HLL_RANDOM_STATE: QualityRandomState = _
```

Source: `src/hash_utils.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Fixed quality hash state used by HyperLogLog sketches.

The seed is part of the HLL wire/storage semantics: serialized sketches only
remain mergeable if every producer uses the same hash state.
