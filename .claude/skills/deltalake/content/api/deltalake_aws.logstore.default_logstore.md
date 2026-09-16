# `deltalake_aws::logstore::default_logstore`

Crate `deltalake-aws` · 2 public items · structured records in [`model/deltalake_aws.logstore.default_logstore.json`](../model/deltalake_aws.logstore.default_logstore.json)

## default_s3_logstore

`function` · `deltalake_aws::logstore::default_logstore::default_s3_logstore`

Also reachable as `deltalake_aws::logstore::default_s3_logstore`

```rust
fn default_s3_logstore(store: deltalake_core::logstore::ObjectStoreRef, root_store: deltalake_core::logstore::ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> std::sync::Arc<dyn LogStore>
```

Return the [S3LogStore] implementation with the provided configuration options

---

## S3LogStore

`struct` · `deltalake_aws::logstore::default_logstore::S3LogStore`

Also reachable as `deltalake_aws::logstore::S3LogStore`

```rust
struct S3LogStore
```

**Implements**: `deltalake_core::logstore::LogStore`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, config: LogStoreConfig) -> Self
```

**via `deltalake_core::logstore::LogStore`**

```rust
async fn abort_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, _operation_id: Uuid) -> Result<(), TransactionError>
fn config(&self) -> &LogStoreConfig
async fn get_latest_version(&self, current_version: Version) -> DeltaResult<Version>
fn name(&self) -> String
fn object_store(&self, _operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
async fn read_commit_entry(&self, version: Version) -> DeltaResult<Option<Bytes>>
fn root_object_store(&self, _operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
async fn write_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, _operation_id: Uuid) -> Result<(), TransactionError>
```

Default [`LogStore`] implementation

---
