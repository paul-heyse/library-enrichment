# `deltalake_core::logstore::factories::LogStoreFactoryRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.factories.LogStoreFactoryRegistry.json).

<a id="op-72a044035fb531ea19f75ea6"></a>
## LogStoreFactoryRegistry

`type_alias` · `deltalake_core::logstore::factories::LogStoreFactoryRegistry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type LogStoreFactoryRegistry = std::sync::Arc<dashmap::DashMap<url::Url, std::sync::Arc<dyn LogStoreFactory>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L99).

Source: `crates/core/src/logstore/factories.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Registry of [`LogStoreFactory`](../operations/deltalake_core.logstore.factories.LogStoreFactory.md#op-2bd01a2a1c90cd84c18ef166) instances
