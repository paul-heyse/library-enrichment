# `deltalake_core::logstore`

Crate `deltalake-core` · 15 public items · structured records in [`model/deltalake_core.logstore.json`](../model/deltalake_core.logstore.json)

## CommitOrBytes

`enum` · `deltalake_core::logstore::CommitOrBytes`

Also reachable as `deltalake::logstore::CommitOrBytes`

```rust
enum CommitOrBytes
```

**Variants**: `TmpCommit`, `LogBytes`

**Derives**: Clone

Holder whether it's tmp_commit path or commit bytes

---

## abort_commit_entry

`function` · `deltalake_core::logstore::abort_commit_entry`

Also reachable as `deltalake::logstore::abort_commit_entry`

```rust
async fn abort_commit_entry(storage: &dyn ObjectStore, _version: kernel::Version, tmp_commit: &object_store::path::Path) -> Result<(), kernel::transaction::TransactionError>
```

Default implementation for aborting a commit entry

---

## default_logstore

`function` · `deltalake_core::logstore::default_logstore`

Also reachable as `deltalake::logstore::default_logstore`

```rust
fn default_logstore(prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> std::sync::Arc<dyn LogStore>
```

Return the [DefaultLogStore] implementation with the provided configuration options

---

## extract_version_from_filename

`function` · `deltalake_core::logstore::extract_version_from_filename`

Also reachable as `deltalake::logstore::extract_version_from_filename`

```rust
fn extract_version_from_filename(name: &str) -> Option<kernel::Version>
```

Extract version from a file name in the delta log

---

## get_actions

`function` · `deltalake_core::logstore::get_actions`

Also reachable as `deltalake::logstore::get_actions`

```rust
fn get_actions(version: kernel::Version, commit_log_bytes: &bytes::Bytes) -> Result<Vec<kernel::Action>, DeltaTableError>
```

Reads a commit and gets list of actions

---

## get_all_versions_from

`function` · `deltalake_core::logstore::get_all_versions_from`

Also reachable as `deltalake::logstore::get_all_versions_from`

```rust
async fn get_all_versions_from(log_store: LogStoreRef, start_version: u64) -> DeltaResult<(Vec<u64>, Vec<kernel::CommitInfo>)>
```

Get all versions related to the delta table and return a tuple of a vector of versions and a
vector of commit infos. We guarantee the length of the two vectors is equal

---

## get_latest_version

`function` · `deltalake_core::logstore::get_latest_version`

Also reachable as `deltalake::logstore::get_latest_version`

```rust
async fn get_latest_version(log_store: &dyn LogStore, current_version: kernel::Version) -> DeltaResult<kernel::Version>
```

Default implementation for retrieving the latest version

---

## logstore_for

`function` · `deltalake_core::logstore::logstore_for`

Also reachable as `deltalake::logstore::logstore_for`

```rust
fn logstore_for(location: &url::Url, storage_config: StorageConfig) -> DeltaResult<LogStoreRef>
```

Return the [LogStoreRef] for the provided [Url] location

This will use the built-in process global [crate::storage::ObjectStoreRegistry] by default

```rust
# use deltalake_core::logstore::*;
# use std::collections::HashMap;
# use url::Url;
let location = Url::parse("memory:///").expect("Failed to make location");
let storage_config = StorageConfig::default();
let logstore = logstore_for(&location, storage_config).expect("Failed to get a logstore");
```

---

## logstore_with

`function` · `deltalake_core::logstore::logstore_with`

Also reachable as `deltalake::logstore::logstore_with`

```rust
fn logstore_with(root_store: ObjectStoreRef, location: &url::Url, storage_config: StorageConfig) -> DeltaResult<LogStoreRef>
```

Return the [LogStoreRef] using the given [ObjectStoreRef]

---

## read_commit_entry

`function` · `deltalake_core::logstore::read_commit_entry`

Also reachable as `deltalake::logstore::read_commit_entry`

```rust
async fn read_commit_entry(storage: &dyn ObjectStore, version: kernel::Version) -> DeltaResult<Option<bytes::Bytes>>
```

Read delta log for a specific version

---

## to_uri

`function` · `deltalake_core::logstore::to_uri`

Also reachable as `deltalake::logstore::to_uri`

```rust
fn to_uri(root: &url::Url, location: &object_store::path::Path) -> String
```

Join the given `root` [Url] with the [Path] to produce a URI (String) of the two together.

This is largely a convenience function to help with the nuances of empty [Path] and file [Url]s

---

## write_commit_entry

`function` · `deltalake_core::logstore::write_commit_entry`

Also reachable as `deltalake::logstore::write_commit_entry`

```rust
async fn write_commit_entry(storage: &dyn ObjectStore, version: kernel::Version, tmp_commit: &object_store::path::Path) -> Result<(), kernel::transaction::TransactionError>
```

Default implementation for writing a commit entry

---

## LogStoreConfig

`struct` · `deltalake_core::logstore::LogStoreConfig`

Also reachable as `deltalake::logstore::LogStoreConfig`

```rust
struct LogStoreConfig
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn decorate_store<T: ObjectStore + Clone>(&self, store: T, table_root: Option<&url::Url>) -> DeltaResult<Box<dyn ObjectStore>>
fn location(&self) -> &Url
fn new(location: &Url, options: StorageConfig) -> Self
fn object_store_factory(&self) -> ObjectStoreFactoryRegistry
fn options(&self) -> &StorageConfig
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

Configuration parameters for a log store

---

## LogStore

`trait` · `deltalake_core::logstore::LogStore`

Also reachable as `deltalake::logstore::LogStore`

```rust
trait LogStore: Send + Sync + AsAny
```

**Implementors** (4)

- `alloc::sync::Arc`
- `deltalake_aws::logstore::default_logstore::S3LogStore`
- `deltalake_core::logstore::default_logstore::DefaultLogStore`
- `deltalake_lakefs::logstore::LakeFSLogStore`

**Methods** (16)

```rust
async fn abort_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
fn config(&self) -> &LogStoreConfig
fn engine(&self, operation_id: Option<Uuid>) -> Arc<dyn Engine>
async fn get_latest_version(&self, start_version: Version) -> DeltaResult<Version>
async fn is_delta_table_location(&self) -> DeltaResult<bool>
fn log_path(&self) -> &Path
fn name(&self) -> String
fn object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
fn object_store_url(&self) -> ObjectStoreUrl
async fn read_commit_entry(&self, version: Version) -> DeltaResult<Option<Bytes>>
async fn refresh(&self) -> DeltaResult<()>
fn root_object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
fn root_url(&self) -> &Url
fn to_uri(&self, location: &Path) -> String
fn transaction_url(&self, _operation_id: Option<Uuid>) -> DeltaResult<Url>
async fn write_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
```

Trait for critical operations required to read and write commit entries in Delta logs.

The correctness is predicated on the atomicity and durability guarantees of
the implementation of this interface. Specifically,

- Atomic visibility: Any commit created via `write_commit_entry` must become visible atomically.
- Mutual exclusion: Only one writer must be able to create a commit for a specific version.
- Consistent listing: Once a commit entry for version `v` has been written, any future call to
  `get_latest_version` must return a version >= `v`, i.e. the underlying file system entry must
  become visible immediately.

---

## LogStoreRef

`type_alias` · `deltalake_core::logstore::LogStoreRef`

Also reachable as `deltalake::logstore::LogStoreRef`

```rust
type LogStoreRef = std::sync::Arc<dyn LogStore>
```

Sharable reference to [`LogStore`]

---
