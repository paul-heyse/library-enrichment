# `deltalake_core::operations::add_column::AddColumnBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.add_column.AddColumnBuilder.json).

<a id="op-60aaeeefeb796e09552ed166"></a>
## AddColumnBuilder

`struct` · `deltalake_core::operations::add_column::AddColumnBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AddColumnBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L23).

Source: `crates/core/src/operations/add_column.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add new columns and/or nested fields to a table

<a id="op-792d21796b45cd8c8b4001d1"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::add_column::AddColumnBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <AddColumnBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L118).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_column.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bf27a1c1116b23c09ab7711"></a>
## Output

`assoc_type` · `deltalake_core::operations::add_column::AddColumnBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_column.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c1167e33d66ffadfb99238c"></a>
## into_future

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L120).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_column.rs:120`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ab8feb8994330976acf6c85"></a>
## with_commit_properties

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [72, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_column.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-589e17c4ded3988e9f508157"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [72, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_column.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-ecaa464d6b1bf4e8424b4678"></a>
## with_fields

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::with_fields` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_fields(self, fields: impl IntoIterator<Item = StructField> + Clone) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L57).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [72, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_column.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the fields to be added

<a id="op-5f981f19591025cdadc4ca9e"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::add_column::AddColumnBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L31).

Source: `crates/core/src/operations/add_column.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-208b8404705c4756b3e46f6e"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::add_column::AddColumnBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L32).

Source: `crates/core/src/operations/add_column.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf33571fd900af470ee04f56"></a>
## fields

`struct_field` · `deltalake_core::operations::add_column::AddColumnBuilder::fields` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
fields: Option<Vec<kernel::StructField>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L27).

Source: `crates/core/src/operations/add_column.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fields to add/merge into schema

<a id="op-f93e4d1b4d7e4d9a36bba85d"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [42, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/add_column.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eee291d26ec590b7871b459"></a>
## log_store

`struct_field` · `deltalake_core::operations::add_column::AddColumnBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L29).

Source: `crates/core/src/operations/add_column.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-ac6df75bac7ce44f1fa45d91"></a>
## log_store

`function` · `deltalake_core::operations::add_column::AddColumnBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_column::AddColumnBuilder", "path": "AddColumnBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [42, 2], "filename": "crates/core/src/operations/add_column.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/add_column.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b1357d82fb301aa1e978d62"></a>
## snapshot

`struct_field` · `deltalake_core::operations::add_column::AddColumnBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_column.rs#L25).

Source: `crates/core/src/operations/add_column.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
