# `deltalake_core::logstore::factories::store_for`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.factories.store_for.json).

<a id="op-523eacee5278d0ceffed70e8"></a>
## store_for

`function` · `deltalake_core::logstore::factories::store_for` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn store_for<K, V, I>(url: &url::Url, options: I) -> DeltaResult<super::ObjectStoreRef> where I: IntoIterator<Item = (K, V)>, K: AsRef<str> + Into<String>, V: AsRef<str> + Into<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L81).

Source: `crates/core/src/logstore/factories.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Simpler access pattern for the [ObjectStoreFactoryRegistry](../operations/deltalake_core.logstore.factories.ObjectStoreFactoryRegistry.md#op-e56e84ed794e72940499bc83) to get a single store
