# ObjectStoreFactory

`deltalake_core::logstore::factories::ObjectStoreFactory`

```rust
trait ObjectStoreFactory: Send + Sync
```

Also reachable as `deltalake::logstore::ObjectStoreFactory`, `deltalake_core::logstore::ObjectStoreFactory`

Prose: [`api/deltalake_core.logstore.factories.md`](../api/deltalake_core.logstore.factories.md#objectstorefactory) · records: [`model/deltalake_core.logstore.factories.json`](../model/deltalake_core.logstore.factories.json)

## Required

Every implementation must supply these.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

## Implementors (9)

Read one before writing your own.

- `deltalake_aws::storage::S3ObjectStoreFactory`
- `deltalake_azure::AzureFactory`
- `deltalake_catalog_unity::UnityCatalogFactory`
- `deltalake_core::logstore::factories::DefaultObjectStoreFactory`
- `deltalake_gcp::GcpFactory`
- `deltalake_hdfs::HdfsFactory`
- `deltalake_lakefs::storage::LakeFSObjectStoreFactory`
- `deltalake_mount::MountFactory`
- `deltalake_opendal::factory::OpendalObjectStoreFactory`

## Documentation

Factory trait for creating [`ObjectStore`](::object_store::ObjectStore) instances at runtime
