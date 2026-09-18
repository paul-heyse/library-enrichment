# `deltalake_aws::logstore::default_logstore::default_s3_logstore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.logstore.default_logstore.default_s3_logstore.json).

<a id="op-7cc33738555035799f27c7f0"></a>
## default_s3_logstore

`function` · `deltalake_aws::logstore::default_logstore::default_s3_logstore` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default_s3_logstore(store: deltalake_core::logstore::ObjectStoreRef, root_store: deltalake_core::logstore::ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> std::sync::Arc<dyn LogStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L15).

Source: `crates/aws/src/logstore/default_logstore.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [S3LogStore](../operations/deltalake_aws.logstore.default_logstore.S3LogStore.md#op-4291bd07320cbe3938f921bb) implementation with the provided configuration options
