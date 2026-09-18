# `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update_table_metadata.UpdateTableMetadataBuilder.json).

<a id="op-1ef6af20a988594f5bfa5922"></a>
## UpdateTableMetadataBuilder

`struct` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UpdateTableMetadataBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L51).

Source: `crates/core/src/operations/update_table_metadata.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update table metadata operation

<a id="op-4c047fa5320b935e3f2128ac"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <UpdateTableMetadataBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L126).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [164, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_table_metadata.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-354fb5541f3974c38a3acd0a"></a>
## Output

`assoc_type` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [164, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_table_metadata.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-658ead4266211fbb8fd3db5f"></a>
## into_future

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [164, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_table_metadata.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d32693364fea35b484f8c7c"></a>
## with_commit_properties

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L91).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [101, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_table_metadata.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-8a704ca72e1441e78691ea10"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [101, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_table_metadata.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-28e01a1f211177e748a2e0b1"></a>
## with_update

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::with_update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_update(self, update: TableMetadataUpdate) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [101, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_table_metadata.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the complete metadata update

<a id="op-16a159eb475e2784d1b03a94"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L59).

Source: `crates/core/src/operations/update_table_metadata.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-12c36fc4598a75ba26348e8b"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L60).

Source: `crates/core/src/operations/update_table_metadata.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-494c4c800dcaae3e22152b84"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [70, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update_table_metadata.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c74650be9f40456c41d797e"></a>
## log_store

`struct_field` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L57).

Source: `crates/core/src/operations/update_table_metadata.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-75f9b92c28aa8afe3d557494"></a>
## log_store

`function` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder", "path": "UpdateTableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [70, 2], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update_table_metadata.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a41488f538191fcf5a0d88c"></a>
## snapshot

`struct_field` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L53).

Source: `crates/core/src/operations/update_table_metadata.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state

<a id="op-fcdb5e06b8e8b9eb25030ec3"></a>
## update

`struct_field` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder::update` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
update: Option<TableMetadataUpdate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L55).

Source: `crates/core/src/operations/update_table_metadata.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The metadata update to apply
