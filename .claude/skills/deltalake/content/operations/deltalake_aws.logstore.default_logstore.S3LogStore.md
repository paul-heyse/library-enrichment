# `deltalake_aws::logstore::default_logstore::S3LogStore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.logstore.default_logstore.S3LogStore.json).

<a id="op-4291bd07320cbe3938f921bb"></a>
## S3LogStore

`struct` · `deltalake_aws::logstore::default_logstore::S3LogStore` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3LogStore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L30).

Source: `crates/aws/src/logstore/default_logstore.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementation

<a id="op-abfb5dde35e6c6360326d014"></a>
## abort_commit_entry

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::abort_commit_entry` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, _operation_id: Uuid) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4adfe8d49ed57716530856cf"></a>
## clone

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::clone` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> S3LogStore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/aws/src/logstore/default_logstore.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e136cd202cdb2f18f4277f0"></a>
## config

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::config` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn config(&self) -> &LogStoreConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L126).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94fdad7bb6c00ea513e8c1e0"></a>
## fmt

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/aws/src/logstore/default_logstore.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2467907b6ecd981a804ba3ff"></a>
## get_latest_version

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::get_latest_version` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_latest_version(&self, current_version: Version) -> DeltaResult<Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89716bb1d4ce7c92b11892b8"></a>
## name

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::name` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38fa129b4910a2e3ad12502a"></a>
## new

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::new` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, config: LogStoreConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [57, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/logstore/default_logstore.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`S3LogStore`](../operations/deltalake_aws.logstore.default_logstore.S3LogStore.md#op-4291bd07320cbe3938f921bb)

# Arguments

* `prefixed_store` - A shared reference to an [`object_store::ObjectStore`]
  with "/" pointing at delta table root (i.e. where `_delta_log` is located).
* `root_store` - A shared reference to an [`object_store::ObjectStore`] with "/"
  pointing at root of the storage system.
* `location` - A url corresponding to the storage location of `storage`.

Unresolved upstream links (retained, not inferred): ``object_store::ObjectStore``.

<a id="op-dbeac36a631912da9177100d"></a>
## object_store

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::object_store` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store(&self, _operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L118).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3257ac9c99f84e5f28cdf39a"></a>
## read_commit_entry

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::read_commit_entry` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_commit_entry(&self, version: Version) -> DeltaResult<Option<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9c39036f296e1b4456e3e56"></a>
## root_object_store

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::root_object_store` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn root_object_store(&self, _operation_id: Option<Uuid>) -> Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L122).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:122`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b8eb49fb6f29da510a6ca8f"></a>
## write_commit_entry

`function` · `deltalake_aws::logstore::default_logstore::S3LogStore::write_commit_entry` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_commit_entry(&self, version: Version, commit_or_bytes: CommitOrBytes, _operation_id: Uuid) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::logstore::default_logstore::S3LogStore", "path": "S3LogStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [129, 2], "filename": "crates/aws/src/logstore/default_logstore.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::LogStore", "path": "LogStore"}, "trait_path": "deltalake_core::logstore::LogStore"}`

Source: `crates/aws/src/logstore/default_logstore.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Tries to commit a prepared commit file. Returns [`TransactionError`](../operations/deltalake_core.kernel.transaction.TransactionError.md#op-c23a31a79225899dcc5279f3)
if the given `version` already exists. The caller should handle the retry logic itself.
This is low-level transaction API. If user does not want to maintain the commit loop then
the `DeltaTransaction.commit` is desired to be used as it handles `try_commit_transaction`
with retry logic.

<a id="op-d933430a1233ced504e44180"></a>
## config

`struct_field` · `deltalake_aws::logstore::default_logstore::S3LogStore::config` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: LogStoreConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L33).

Source: `crates/aws/src/logstore/default_logstore.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23aaecc621d413c7e9c2d0aa"></a>
## prefixed_store

`struct_field` · `deltalake_aws::logstore::default_logstore::S3LogStore::prefixed_store` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
prefixed_store: deltalake_core::logstore::ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L31).

Source: `crates/aws/src/logstore/default_logstore.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-372861152b86df6c5efe3506"></a>
## root_store

`struct_field` · `deltalake_aws::logstore::default_logstore::S3LogStore::root_store` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
root_store: deltalake_core::logstore::ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/default_logstore.rs#L32).

Source: `crates/aws/src/logstore/default_logstore.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
