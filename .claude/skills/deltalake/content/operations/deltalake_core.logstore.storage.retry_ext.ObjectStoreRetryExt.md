# `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.retry_ext.ObjectStoreRetryExt.json).

<a id="op-49a82e6694d454f6ac1ca1b4"></a>
## ObjectStoreRetryExt

`trait` · `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ObjectStoreRetryExt: ObjectStore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/retry_ext.rs#L27).

Source: `crates/core/src/logstore/storage/retry_ext.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

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

Unresolved upstream links (retained, not inferred): ``ObjectStore``.

<a id="op-77be15fa6ddf1213d96cbf31"></a>
## delete_with_retries

`function` · `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt::delete_with_retries` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn delete_with_retries(&self, location: &Path, max_retries: usize) -> Result<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/retry_ext.rs#L66).

Source: `crates/core/src/logstore/storage/retry_ext.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete the object at the specified location

<a id="op-30d83bf072011b4f5dfab5c7"></a>
## put_with_retries

`function` · `deltalake_core::logstore::storage::retry_ext::ObjectStoreRetryExt::put_with_retries` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_with_retries(&self, location: &Path, bytes: PutPayload, max_retries: usize) -> Result<PutResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/retry_ext.rs#L34).

Source: `crates/core/src/logstore/storage/retry_ext.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Save the provided bytes to the specified location

The operation is guaranteed to be atomic, it will either successfully write the entirety of
bytes to location, or fail. No clients should be able to observe a partially written object

Note that `put_with_opts` may have precondition semantics, and thus may not be retriable.
