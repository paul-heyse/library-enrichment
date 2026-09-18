# `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.drop_column_not_null.DropColumnNotNullBuilder.json).

<a id="op-0d8f0aec0efbbc9818ce2ef5"></a>
## DropColumnNotNullBuilder

`struct` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DropColumnNotNullBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L25).

Source: `crates/core/src/operations/drop_column_not_null.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drop the `NOT NULL` constraint on a top-level column, making it nullable.

<a id="op-66d18c2f4294ad64a6252a59"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <DropColumnNotNullBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L140).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [172, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_column_not_null.rs:140`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba1104da68a37c21c75e6331"></a>
## Output

`assoc_type` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [172, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_column_not_null.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e67b2aa5c621055a9ccb790"></a>
## into_future

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [172, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_column_not_null.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5475257dc2439c916f12da52"></a>
## with_column

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::with_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_column(self, column_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [75, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_column_not_null.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the column whose `NOT NULL` constraint should be dropped

<a id="op-538cd06c6cb136442571df8e"></a>
## with_commit_properties

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [75, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_column_not_null.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-4774e0f3d22c35f5a3681111"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [75, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_column_not_null.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-3854719de0ca61b9dd9cd1c8"></a>
## column_name

`struct_field` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::column_name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
column_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L29).

Source: `crates/core/src/operations/drop_column_not_null.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the column whose `NOT NULL` constraint should be dropped

<a id="op-21a57e92eeb1793e134fa8c2"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L33).

Source: `crates/core/src/operations/drop_column_not_null.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-616490ecf9f64145d1daccfa"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L34).

Source: `crates/core/src/operations/drop_column_not_null.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2b5a56ffe82f90fbccb122b"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [44, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/drop_column_not_null.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92fbaaa6566445988007d6b1"></a>
## log_store

`function` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder", "path": "DropColumnNotNullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [44, 2], "filename": "crates/core/src/operations/drop_column_not_null.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/drop_column_not_null.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef74dc8c9a2f9c38f38cfa55"></a>
## log_store

`struct_field` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L31).

Source: `crates/core/src/operations/drop_column_not_null.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-bd4a01225f0ce964c0a892ff"></a>
## snapshot

`struct_field` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L27).

Source: `crates/core/src/operations/drop_column_not_null.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
