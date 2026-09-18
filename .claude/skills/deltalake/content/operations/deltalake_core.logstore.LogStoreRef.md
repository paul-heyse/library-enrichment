# `deltalake_core::logstore::LogStoreRef`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.LogStoreRef.json).

<a id="op-1b56118828480c7f5e758964"></a>
## LogStoreRef

`type_alias` · `deltalake_core::logstore::LogStoreRef` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type LogStoreRef = std::sync::Arc<dyn LogStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L161).

Source: `crates/core/src/logstore/mod.rs:161`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sharable reference to [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639)
