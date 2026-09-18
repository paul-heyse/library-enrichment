# `deltalake_opendal::adapter::OpendalAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.adapter.OpendalAdapter.json).

<a id="op-710e9b27c5cc68f393fd433d"></a>
## OpendalAdapter

`trait` · `deltalake_opendal::adapter::OpendalAdapter` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait OpendalAdapter: Send + Sync + std::fmt::Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L30).

Source: `crates/opendal/src/adapter.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Per-service specialization for the generic OpenDAL factories.

Only three things vary between OpenDAL services: how a delta URL plus storage
options map onto an [`OperatorSpec`](../operations/deltalake_opendal.adapter.OperatorSpec.md#op-3eb3c99b88cef66f9b9f4a35), whether the resulting store needs
wrapping, and what prefix the log store should use. Everything else is shared
by [`crate::factory`].

Unresolved upstream links (retained, not inferred): ``crate::factory``.

<a id="op-54c17f21a4d0b21303a489b1"></a>
## logstore_prefix

`function` · `deltalake_opendal::adapter::OpendalAdapter::logstore_prefix` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logstore_prefix(&self, spec: &OperatorSpec) -> Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L42).

Source: `crates/opendal/src/adapter.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The prefix the log store's `PrefixStore` should apply on top of the root
store. Defaults to [`OperatorSpec::table_prefix`](../operations/deltalake_opendal.adapter.OperatorSpec.md#op-8fa47be7bb0dbd6fe45eecdd).

<a id="op-5b04d582c9c796ca572731b7"></a>
## resolve

`function` · `deltalake_opendal::adapter::OpendalAdapter::resolve` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn resolve(&self, url: &Url, config: &StorageConfig) -> DeltaResult<OperatorSpec>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L32).

Source: `crates/opendal/src/adapter.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Map a delta table URL and storage options to an [`OperatorSpec`](../operations/deltalake_opendal.adapter.OperatorSpec.md#op-3eb3c99b88cef66f9b9f4a35).

<a id="op-fcac1bda3c66602ef37186ec"></a>
## wrap_store

`function` · `deltalake_opendal::adapter::OpendalAdapter::wrap_store` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn wrap_store(&self, store: ObjectStoreRef, _spec: &OperatorSpec) -> ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L36).

Source: `crates/opendal/src/adapter.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optionally wrap the raw `OpendalStore`. The default is the identity; an
adapter may override this to rewrite paths or otherwise decorate the store.
