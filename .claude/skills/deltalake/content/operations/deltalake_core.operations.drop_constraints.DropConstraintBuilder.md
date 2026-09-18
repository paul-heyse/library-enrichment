# `deltalake_core::operations::drop_constraints::DropConstraintBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.drop_constraints.DropConstraintBuilder.json).

<a id="op-26439e74a0201221fbe2c3f2"></a>
## DropConstraintBuilder

`struct` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DropConstraintBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L17).

Source: `crates/core/src/operations/drop_constraints.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Remove constraints from the table

<a id="op-671e937a4be11ec2809dc374"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <DropConstraintBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [151, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_constraints.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99ddcc79315dd2eb57dfe025"></a>
## Output

`assoc_type` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [151, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_constraints.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0d4795e2128ce97827e491"></a>
## into_future

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [151, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/drop_constraints.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3e72cebe79bf436466c55fd"></a>
## with_commit_properties

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [76, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_constraints.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-6dcc1c231a8d3fe14e29a93f"></a>
## with_constraint

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::with_constraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_constraint<S: Into<String>>(self, name: S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L54).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [76, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_constraints.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the constraint to be removed

<a id="op-65dfd1efafc46f518e004911"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [76, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_constraints.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-a637967c897f0cdc6cb47d89"></a>
## with_raise_if_not_exists

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::with_raise_if_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_raise_if_not_exists(self, raise: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [76, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/drop_constraints.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify if you want to raise if the constraint does not exist

<a id="op-39ce470f849d4cedd56ba7be"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L27).

Source: `crates/core/src/operations/drop_constraints.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-ca086344f6f891d2cd585d5b"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L28).

Source: `crates/core/src/operations/drop_constraints.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebd29b7e33b312374e8f957a"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [38, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/drop_constraints.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4948861a00aff13b693137df"></a>
## log_store

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L25).

Source: `crates/core/src/operations/drop_constraints.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-cab4bab13c1c992f13ff37d0"></a>
## log_store

`function` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::drop_constraints::DropConstraintBuilder", "path": "DropConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [38, 2], "filename": "crates/core/src/operations/drop_constraints.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/drop_constraints.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-913638d7a7ed7d89753cf4ae"></a>
## name

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L21).

Source: `crates/core/src/operations/drop_constraints.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the constraint

<a id="op-fd5c34464208a5486e0aba61"></a>
## raise_if_not_exists

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::raise_if_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
raise_if_not_exists: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L23).

Source: `crates/core/src/operations/drop_constraints.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Raise if constraint doesn't exist

<a id="op-69db23a2237d64282ec205d4"></a>
## snapshot

`struct_field` · `deltalake_core::operations::drop_constraints::DropConstraintBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_constraints.rs#L19).

Source: `crates/core/src/operations/drop_constraints.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
