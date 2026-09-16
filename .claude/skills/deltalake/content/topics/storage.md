# Storage and log stores

A `LogStore` is what makes commits atomic, and it is separate from the object store that holds data files. The split exists because atomic rename is not universal: S3 historically needed external coordination, which is why a backend is registered as a factory pair rather than a single store. Registration is by URL scheme, so a backend that is not registered fails at open time with an unhelpful scheme error.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend` | struct | 16 | [prose](../api/deltalake_core.logstore.storage.runtime.md#deltaiostoragebackend) | [records](../model/deltalake_core.logstore.storage.runtime.json) |
| `deltalake_aws::storage::S3StorageBackend` | struct | 13 | [prose](../api/deltalake_aws.storage.md#s3storagebackend) | [records](../model/deltalake_aws.storage.json) |
| `deltalake_aws::logstore::default_logstore::S3LogStore` | struct | 11 | [prose](../api/deltalake_aws.logstore.default_logstore.md#s3logstore) | [records](../model/deltalake_aws.logstore.default_logstore.json) |
| `deltalake_opendal::shim::ConditionalPutShim` | struct | 13 | [prose](../api/deltalake_opendal.shim.md#conditionalputshim) | [records](../model/deltalake_opendal.shim.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::logstore::LogStore` | 8 | 8 | 4 | [LogStore](../traits/LogStore.md) |
| `deltalake_core::logstore::factories::LogStoreFactory` | 1 | 0 | 9 | [LogStoreFactory](../traits/LogStoreFactory.md) |
| `deltalake_core::logstore::factories::ObjectStoreFactory` | 1 | 0 | 9 | [ObjectStoreFactory](../traits/ObjectStoreFactory.md) |
| `deltalake_core::logstore::storage::ObjectStoreRegistry` | 2 | 0 | 1 | [ObjectStoreRegistry](../traits/ObjectStoreRegistry.md) |
| `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt` | 0 | 2 | 0 | [ObjectStoreRetryExt](../traits/ObjectStoreRetryExt.md) |
| `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl` | 1 | 0 | 3 | [AsObjectStoreUrl](../traits/AsObjectStoreUrl.md) |
| `deltalake_opendal::adapter::OpendalAdapter` | 1 | 2 | 1 | [OpendalAdapter](../traits/OpendalAdapter.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Register the backend crate's handlers before opening a URL with its scheme; nothing does it implicitly.
- On S3, confirm whether the deployment provides conditional put or needs a locking provider -- the two configurations fail very differently.

## Anti-patterns

- Putting credentials in the table URL instead of storage options.
- Assuming a scheme is supported because the crate is a dependency. It has to be registered.

## Agent checklist

- Are the backend handlers registered for every scheme in use?
- Is concurrent-writer safety established for this store, not assumed?
