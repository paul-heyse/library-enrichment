# LogStore

`deltalake_core::logstore::LogStore`

```rust
trait LogStore: Send + Sync + AsAny
```

Also reachable as `deltalake::logstore::LogStore`

Prose: [`api/deltalake_core.logstore.md`](../api/deltalake_core.logstore.md#logstore) · records: [`model/deltalake_core.logstore.json`](../model/deltalake_core.logstore.json)

## Required

Every implementation must supply these.

```rust
async fn abort_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
fn config(&self) -> &LogStoreConfig
async fn get_latest_version(&self, start_version: Version) -> DeltaResult<Version>
fn name(&self) -> String
fn object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
async fn read_commit_entry(&self, version: Version) -> DeltaResult<Option<Bytes>>
fn root_object_store(&self, operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
async fn write_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, operation_id: Uuid) -> Result<(), TransactionError>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn engine(&self, operation_id: Option<Uuid>) -> Arc<dyn Engine>
async fn is_delta_table_location(&self) -> DeltaResult<bool>
fn log_path(&self) -> &Path
fn object_store_url(&self) -> ObjectStoreUrl
async fn refresh(&self) -> DeltaResult<()>
fn root_url(&self) -> &Url
fn to_uri(&self, location: &Path) -> String
fn transaction_url(&self, _operation_id: Option<Uuid>) -> DeltaResult<Url>
```

## Implementors (4)

Read one before writing your own.

- `alloc::sync::Arc`
- `deltalake_aws::logstore::default_logstore::S3LogStore`
- `deltalake_core::logstore::default_logstore::DefaultLogStore`
- `deltalake_lakefs::logstore::LakeFSLogStore`

## Demonstrated by 1 upstream example(s)

- [`corpus/tests/it_datafusion/command_optimize.rs`](../corpus/tests/it_datafusion/command_optimize.rs)

## Documentation

Trait for critical operations required to read and write commit entries in Delta logs.

The correctness is predicated on the atomicity and durability guarantees of
the implementation of this interface. Specifically,

- Atomic visibility: Any commit created via `write_commit_entry` must become visible atomically.
- Mutual exclusion: Only one writer must be able to create a commit for a specific version.
- Consistent listing: Once a commit entry for version `v` has been written, any future call to
  `get_latest_version` must return a version >= `v`, i.e. the underlying file system entry must
  become visible immediately.
