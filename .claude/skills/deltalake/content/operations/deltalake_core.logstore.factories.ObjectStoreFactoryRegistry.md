# `deltalake_core::logstore::factories::ObjectStoreFactoryRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.factories.ObjectStoreFactoryRegistry.json).

<a id="op-e56e84ed794e72940499bc83"></a>
## ObjectStoreFactoryRegistry

`type_alias` · `deltalake_core::logstore::factories::ObjectStoreFactoryRegistry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ObjectStoreFactoryRegistry = std::sync::Arc<dashmap::DashMap<url::Url, std::sync::Arc<dyn ObjectStoreFactory>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L14).

Source: `crates/core/src/logstore/factories.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Factory registry to manage [`ObjectStoreFactory`](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) instances
