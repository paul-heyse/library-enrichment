# LogStoreFactory

`deltalake_core::logstore::factories::LogStoreFactory`

```rust
trait LogStoreFactory: Send + Sync
```

Also reachable as `deltalake::logstore::LogStoreFactory`, `deltalake_core::logstore::LogStoreFactory`

Prose: [`api/deltalake_core.logstore.factories.md`](../api/deltalake_core.logstore.factories.md#logstorefactory) · records: [`model/deltalake_core.logstore.factories.json`](../model/deltalake_core.logstore.factories.json)

## Required

Every implementation must supply these.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

## Implementors (9)

Read one before writing your own.

- `deltalake_aws::S3LogStoreFactory`
- `deltalake_azure::AzureFactory`
- `deltalake_catalog_unity::UnityCatalogFactory`
- `deltalake_core::logstore::factories::DefaultLogStoreFactory`
- `deltalake_gcp::GcpFactory`
- `deltalake_hdfs::HdfsFactory`
- `deltalake_lakefs::LakeFSLogStoreFactory`
- `deltalake_mount::MountFactory`
- `deltalake_opendal::factory::OpendalLogStoreFactory`

## Documentation

Trait for generating [LogStore] implementations
