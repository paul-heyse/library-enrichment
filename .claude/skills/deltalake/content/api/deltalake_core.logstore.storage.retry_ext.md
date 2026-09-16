# `deltalake_core::logstore::storage::retry_ext`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.logstore.storage.retry_ext.json`](../model/deltalake_core.logstore.storage.retry_ext.json)

## ObjectStoreRetryExt

`trait` · `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt`

Also reachable as `deltalake::logstore::ObjectStoreRetryExt`, `deltalake_core::logstore::ObjectStoreRetryExt`

```rust
trait ObjectStoreRetryExt: ObjectStore
```

**Methods** (2)

```rust
async fn delete_with_retries(&self, location: &Path, max_retries: usize) -> Result<()>
async fn put_with_retries(&self, location: &Path, bytes: PutPayload, max_retries: usize) -> Result<PutResult>
```

Retry extension for [`ObjectStore`]

Read-only operations are retried by [`ObjectStore`] internally. However, PUT/DELETE operations
are not retried even thought they are technically idempotent. [`ObjectStore`] does not retry
those operations because having preconditions may produce different results for the same
request. PUT/DELETE operations without preconditions are idempotent and can be retried.
Unfortunately, [`ObjectStore`]'s retry mechanism only works on HTTP request level, thus there
is no way to distinguish whether a request has preconditions or not.

This trait provides additional methods for working with [`ObjectStore`] that automatically retry
unconditional operations when they fail.

See also:
- https://github.com/apache/arrow-rs/pull/5278

---
