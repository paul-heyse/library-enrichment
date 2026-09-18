# `deltalake_core::operations::restore::RestoreBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.restore.RestoreBuilder.json).

<a id="op-3e336843be6cb6c5579089a2"></a>
## RestoreBuilder

`struct` · `deltalake_core::operations::restore::RestoreBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RestoreBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L81).

Source: `crates/core/src/operations/restore.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restore a Delta table with given version
See this module's documentation for more information

<a id="op-3848a07725710d4f9ed91ca6"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::restore::RestoreBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <RestoreBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [412, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/restore.rs:376`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7a1ac890da137f644102e9"></a>
## Output

`assoc_type` · `deltalake_core::operations::restore::RestoreBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, RestoreMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L375).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [412, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/restore.rs:375`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11109871439f0c06f3349869"></a>
## into_future

`function` · `deltalake_core::operations::restore::RestoreBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L378).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [412, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/restore.rs:378`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1fe66e57338ca3032c23759"></a>
## with_commit_properties

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L149).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:149`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-43be1a6df32f421f9e46546f"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-6cb55737571efda569bb30ec"></a>
## with_datetime_to_restore

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_datetime_to_restore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_datetime_to_restore(self, datetime: DateTime<Utc>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the datetime to restore

<a id="op-e7623e4b60f0e4a157821d16"></a>
## with_ignore_missing_files

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_ignore_missing_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_ignore_missing_files(self, ignore_missing_files: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L137).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:137`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set whether to ignore missing files which delete manually or by vacuum.
If true, continue to run when encountering missing files.

<a id="op-f8b6f175d9e099b118635950"></a>
## with_protocol_downgrade_allowed

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_protocol_downgrade_allowed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_protocol_downgrade_allowed(self, protocol_downgrade_allowed: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set whether allow to downgrade protocol

<a id="op-45dc160b98acde43d1355b28"></a>
## with_version_to_restore

`function` · `deltalake_core::operations::restore::RestoreBuilder::with_version_to_restore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_version_to_restore(self, version: Version) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [159, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/restore.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the version to restore

<a id="op-6c4f40b9af490909ac062ebe"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L95).

Source: `crates/core/src/operations/restore.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-02816e2de3b096af298e3358"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L96).

Source: `crates/core/src/operations/restore.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f551bd40537b366e05808df3"></a>
## datetime_to_restore

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::datetime_to_restore` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
datetime_to_restore: Option<chrono::DateTime<chrono::Utc>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L89).

Source: `crates/core/src/operations/restore.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datetime to restore

<a id="op-acdfee4ccb2936458d652d86"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::restore::RestoreBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [106, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/restore.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a46896d9c3f34a75a0c90952"></a>
## ignore_missing_files

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::ignore_missing_files` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
ignore_missing_files: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L91).

Source: `crates/core/src/operations/restore.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Ignore missing files

<a id="op-568ec7580e0a9827766b5578"></a>
## log_store

`function` · `deltalake_core::operations::restore::RestoreBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreBuilder", "path": "RestoreBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [106, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/restore.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea7addb2e693302d75f31ea2"></a>
## log_store

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L85).

Source: `crates/core/src/operations/restore.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-4fa44754ea20bc0b149657be"></a>
## protocol_downgrade_allowed

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::protocol_downgrade_allowed` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol_downgrade_allowed: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L93).

Source: `crates/core/src/operations/restore.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Protocol downgrade allowed

<a id="op-f151653756e8ce6bbba4f376"></a>
## snapshot

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L83).

Source: `crates/core/src/operations/restore.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-restored table's state

<a id="op-652d7b94b759f0792a75d06f"></a>
## version_to_restore

`struct_field` · `deltalake_core::operations::restore::RestoreBuilder::version_to_restore` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version_to_restore: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L87).

Source: `crates/core/src/operations/restore.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version to restore
