# `deltalake_aws`

Crate `deltalake-aws` · 2 public items · structured records in [`model/deltalake_aws.json`](../model/deltalake_aws.json)

## register_handlers

`function` · `deltalake_aws::register_handlers`

Also reachable as `deltalake::aws::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common S3 url schemes.

[ObjectStoreFactory]: deltalake_core::logstore::ObjectStoreFactory

---

## S3LogStoreFactory

`struct` · `deltalake_aws::S3LogStoreFactory`

Also reachable as `deltalake::aws::S3LogStoreFactory`

```rust
struct S3LogStoreFactory
```

**Implements**: `deltalake_aws::storage::S3StorageOptionsConversion`, `deltalake_core::logstore::factories::LogStoreFactory`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::LogStoreFactory`**

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

---
