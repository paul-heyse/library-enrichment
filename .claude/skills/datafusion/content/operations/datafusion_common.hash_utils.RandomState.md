# `datafusion_common::hash_utils::RandomState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.RandomState.json).

<a id="op-77ab905fd5f9b0d866fa207d"></a>
## RandomState

`type_alias` · `datafusion_common::hash_utils::RandomState` · datafusion-common 55.1.0

```rust
type RandomState = foldhash::fast::FixedState
```

Source: `src/hash_utils.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`RandomState`](../operations/datafusion_common.hash_utils.RandomState.md#op-77ab905fd5f9b0d866fa207d) is optimized for speed and suitable for hash tables and
bloom filters. [`QualityRandomState`](../operations/datafusion_common.hash_utils.QualityRandomState.md#op-228fd7fbe8a1f8017142119b) is optimized for statistical quality
and suitable for algorithms such as HyperLogLog. The tradeoff is that the
fast variant gives up some statistical quality, while the quality variant
is slightly slower.

See: <https://docs.rs/foldhash/0.2.0/src/foldhash/lib.rs.html#17-21>
