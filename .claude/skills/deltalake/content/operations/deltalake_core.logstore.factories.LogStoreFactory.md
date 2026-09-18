# `deltalake_core::logstore::factories::LogStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.factories.LogStoreFactory.json).

<a id="op-2bd01a2a1c90cd84c18ef166"></a>
## LogStoreFactory

`trait` · `deltalake_core::logstore::factories::LogStoreFactory` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait LogStoreFactory: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L102).

Source: `crates/core/src/logstore/factories.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for generating [LogStore](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementations

<a id="op-9fc7c28bf30f5639e9acbeba"></a>
## with_options

`function` · `deltalake_core::logstore::factories::LogStoreFactory::with_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L113).

Source: `crates/core/src/logstore/factories.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) from options.

This method is responsible for creating a new instance of the [LogStore](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementation.

## Parameters
- `prefixed_store`: A reference to the object store.
- `location`: A reference to the URL of the location.
- `options`: A reference to the storage configuration options.

It returns a [DeltaResult](../operations/deltalake_core.errors.DeltaResult.md#op-0ab2063e00e7d8f4da456811) containing an [Arc] to the newly created [LogStore](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementation.

Unresolved upstream links (retained, not inferred): `Arc`.
