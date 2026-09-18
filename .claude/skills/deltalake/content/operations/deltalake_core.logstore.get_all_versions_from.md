# `deltalake_core::logstore::get_all_versions_from`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.get_all_versions_from.json).

<a id="op-6a0490c7e47903d9974ad7a4"></a>
## get_all_versions_from

`function` · `deltalake_core::logstore::get_all_versions_from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_all_versions_from(log_store: LogStoreRef, start_version: u64) -> DeltaResult<(Vec<u64>, Vec<kernel::CommitInfo>)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L812).

Source: `crates/core/src/logstore/mod.rs:812`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get all versions related to the delta table and return a tuple of a vector of versions and a
vector of commit infos. We guarantee the length of the two vectors is equal
