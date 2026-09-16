# `deltalake_lakefs::storage`

Crate `deltalake-lakefs` · 1 public items · structured records in [`model/deltalake_lakefs.storage.json`](../model/deltalake_lakefs.storage.json)

## LakeFSObjectStoreFactory

`struct` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory`

```rust
struct LakeFSObjectStoreFactory
```

**Implements**: `deltalake_core::logstore::factories::ObjectStoreFactory`, `deltalake_lakefs::storage::S3StorageOptionsConversion`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::ObjectStoreFactory`**

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

---
