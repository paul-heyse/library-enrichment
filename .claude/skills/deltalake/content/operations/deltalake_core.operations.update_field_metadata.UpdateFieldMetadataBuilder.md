# `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update_field_metadata.UpdateFieldMetadataBuilder.json).

<a id="op-428328bf172d86d852e8b9eb"></a>
## UpdateFieldMetadataBuilder

`struct` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UpdateFieldMetadataBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L22).

Source: `crates/core/src/operations/update_field_metadata.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update a field's metadata in a schema. If the key does not exists, the entry is inserted.

<a id="op-b3a52ff7ab834d79f226c849"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <UpdateFieldMetadataBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [181, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_field_metadata.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27c672b97f12a0c812fd954b"></a>
## Output

`assoc_type` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [181, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_field_metadata.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c37857d0eb8e916a664c5d83"></a>
## into_future

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [181, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update_field_metadata.rs:150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48276dca4ce14cff7578a22e"></a>
## with_commit_properties

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_field_metadata.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-2829c1fb6c3998d5f1835bcc"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_field_metadata.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-acbb5e2cfd9b3072fa3f9596"></a>
## with_field_name

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::with_field_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_field_name(self, field_name: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_field_metadata.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the field you want to update the metadata for

<a id="op-c88bc3234ce987e7d24fd2fb"></a>
## with_metadata

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::with_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_metadata(self, metadata: HashMap<String, MetadataValue>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update_field_metadata.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the metadata to be added or modified on a field

<a id="op-0bc22a1194fa5ddad483583c"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L32).

Source: `crates/core/src/operations/update_field_metadata.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-f828892ca3232ad9a4ce0550"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L33).

Source: `crates/core/src/operations/update_field_metadata.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aa1e80e85a577cefc6f4def"></a>
## field_name

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::field_name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
field_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L26).

Source: `crates/core/src/operations/update_field_metadata.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the field where the metadata may be updated

<a id="op-f515d306c347c4aa12a66140"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [43, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update_field_metadata.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f7bc1ed590c423084df24d6"></a>
## log_store

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L30).

Source: `crates/core/src/operations/update_field_metadata.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-c65d4022beca39b7b5bf04ec"></a>
## log_store

`function` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder", "path": "UpdateFieldMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [43, 2], "filename": "crates/core/src/operations/update_field_metadata.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update_field_metadata.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc7e8cedf93e1dd638113888"></a>
## metadata

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata: std::collections::HashMap<String, delta_kernel::schema::MetadataValue>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L28).

Source: `crates/core/src/operations/update_field_metadata.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

HashMap of the metadata to upsert

<a id="op-fe57667053e74c5a15cd509c"></a>
## snapshot

`struct_field` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_field_metadata.rs#L24).

Source: `crates/core/src/operations/update_field_metadata.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
