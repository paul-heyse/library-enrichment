# `deltalake_core::logstore::LogStore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.LogStore.json).

<a id="op-05065cf369d14ac5f8039639"></a>
## LogStore

`trait` · `deltalake_core::logstore::LogStore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait LogStore: Send + Sync + AsAny
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L373).

Source: `crates/core/src/logstore/mod.rs:373`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for critical operations required to read and write commit entries in Delta logs.

The correctness is predicated on the atomicity and durability guarantees of
the implementation of this interface. Specifically,

- Atomic visibility: Any commit created via `write_commit_entry` must become visible atomically.
- Mutual exclusion: Only one writer must be able to create a commit for a specific version.
- Consistent listing: Once a commit entry for version `v` has been written, any future call to
  `get_latest_version` must return a version >= `v`, i.e. the underlying file system entry must
  become visible immediately.

<a id="op-ad443984e672f2edd8c7e8d0"></a>
## abort_commit_entry

`function` · `deltalake_core::logstore::LogStore::abort_commit_entry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L397).

Source: `crates/core/src/logstore/mod.rs:397`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Abort the commit entry for the given version.

<a id="op-1b81c313f643c07cf88abcba"></a>
## config

`function` · `deltalake_core::logstore::LogStore::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn config(&self) -> &LogStoreConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L478).

Source: `crates/core/src/logstore/mod.rs:478`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get configuration representing configured log store.

<a id="op-defc876ee41d20be7d177ae3"></a>
## engine

`function` · `deltalake_core::logstore::LogStore::engine` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn engine(&self, operation_id: Option<Uuid>) -> Arc<dyn Engine>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L415).

Source: `crates/core/src/logstore/mod.rs:415`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a kernel [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) backed by this log store's root object store.

<a id="op-716c8cb00dde5d65f1333a8c"></a>
## get_latest_version

`function` · `deltalake_core::logstore::LogStore::get_latest_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_latest_version(&self, start_version: Version) -> DeltaResult<Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L405).

Source: `crates/core/src/logstore/mod.rs:405`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Find latest version currently stored in the delta log.

<a id="op-af69f4ae6699b2dcbece902f"></a>
## is_delta_table_location

`function` · `deltalake_core::logstore::LogStore::is_delta_table_location` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn is_delta_table_location(&self) -> DeltaResult<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L445).

Source: `crates/core/src/logstore/mod.rs:445`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if the location is a delta table location

<a id="op-36204e9b780498088413e7d0"></a>
## log_path

`function` · `deltalake_core::logstore::LogStore::log_path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_path(&self) -> &Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L432).

Source: `crates/core/src/logstore/mod.rs:432`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

[Path] to Delta log

Unresolved upstream links (retained, not inferred): `Path`.

<a id="op-a92b79b2311e55cbb0ec62c6"></a>
## name

`function` · `deltalake_core::logstore::LogStore::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L375).

Source: `crates/core/src/logstore/mod.rs:375`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the name of this LogStore implementation

<a id="op-0198126cbb978386b9fb33f9"></a>
## object_store

`function` · `deltalake_core::logstore::LogStore::object_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L408).

Source: `crates/core/src/logstore/mod.rs:408`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get object store, can pass operation_id for object stores linked to an operation

<a id="op-fe2473b571dfdacfee9b8dd4"></a>
## object_store_url

`function` · `deltalake_core::logstore::LogStore::object_store_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store_url(&self) -> ObjectStoreUrl
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L490).

Source: `crates/core/src/logstore/mod.rs:490`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generate a unique enough url to identify the store in datafusion.
The DF object store registry only cares about the scheme and the host of the url for
registering/fetching. In our case the scheme is hard-coded to "delta-rs", so to get a unique
host we convert the location from this `LogStore` to a valid name, combining the
original scheme, host and path with invalid characters replaced.

This is a legacy/migration helper for delta-rs DataFusion integrations that use a
synthetic per-table `delta-rs://...` URL mapping to a table-scoped (prefixed) object store.
It will not work correctly with fully-qualified file URLs (e.g. shallow clones).

<a id="op-588b23afe8a4c58de3f58cb7"></a>
## read_commit_entry

`function` · `deltalake_core::logstore::LogStore::read_commit_entry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_commit_entry(&self, version: Version) -> DeltaResult<Option<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L383).

Source: `crates/core/src/logstore/mod.rs:383`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read data for commit entry with the given version.

<a id="op-c4296f8375e4a7013cea0c5b"></a>
## refresh

`function` · `deltalake_core::logstore::LogStore::refresh` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn refresh(&self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L378).

Source: `crates/core/src/logstore/mod.rs:378`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trigger sync operation on log store to.

<a id="op-a3a091cfdafd930bd9c5fd1a"></a>
## root_object_store

`function` · `deltalake_core::logstore::LogStore::root_object_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn root_object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L412).

Source: `crates/core/src/logstore/mod.rs:412`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the object store rooted at the table root (not the delta log), optionally scoped to
an operation.

<a id="op-ea1cc547568423794e8b5081"></a>
## root_url

`function` · `deltalake_core::logstore::LogStore::root_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn root_url(&self) -> &Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L427).

Source: `crates/core/src/logstore/mod.rs:427`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get fully qualified uri for table root

<a id="op-383cd058626ee6358b13f45e"></a>
## to_uri

`function` · `deltalake_core::logstore::LogStore::to_uri` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_uri(&self, location: &Path) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L421).

Source: `crates/core/src/logstore/mod.rs:421`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

[Path] to Delta log

Unresolved upstream links (retained, not inferred): `Path`.

<a id="op-f2dce631d951082d90907558"></a>
## transaction_url

`function` · `deltalake_core::logstore::LogStore::transaction_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transaction_url(&self, _operation_id: Option<Uuid>) -> DeltaResult<Url>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L440).

Source: `crates/core/src/logstore/mod.rs:440`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generate the appropriate [Url] to use for executing an operation.

 This can be useful for branching LogStore implementations such as LakeFS which may return
 something other than the base URL.

Unresolved upstream links (retained, not inferred): `Url`.

<a id="op-8c1a621a395047b4abdf36b3"></a>
## write_commit_entry

`function` · `deltalake_core::logstore::LogStore::write_commit_entry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L389).

Source: `crates/core/src/logstore/mod.rs:389`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write list of actions as delta commit entry for given version.

This operation can be retried with a higher version in case the write
fails with [`TransactionError::VersionAlreadyExists`](../operations/deltalake_core.kernel.transaction.TransactionError.md#op-ab277057e0fcfe546a0db97b).
