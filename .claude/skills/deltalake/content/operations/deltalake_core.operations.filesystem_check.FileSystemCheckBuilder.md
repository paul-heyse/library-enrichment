# `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.filesystem_check.FileSystemCheckBuilder.json).

<a id="op-e9c2fdf36333219a95322e63"></a>
## FileSystemCheckBuilder

`struct` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileSystemCheckBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L43).

Source: `crates/core/src/operations/filesystem_check.rs:43`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Audit the Delta Table's active files with the underlying file system.
See this module's documentation for more information

<a id="op-b86e38908a2200e710618419"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <FileSystemCheckBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L256).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [304, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/filesystem_check.rs:256`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45e4fee27d345f6eb907d916"></a>
## Output

`assoc_type` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, FileSystemCheckMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L255).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [304, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/filesystem_check.rs:255`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0138586a84fe151ae633e6d"></a>
## into_future

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [304, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/filesystem_check.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47a2b00e21dc7b5f7001d560"></a>
## with_commit_properties

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L132).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [196, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/filesystem_check.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to write to the commit

<a id="op-b3ba3346905a26b279b591a5"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [196, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/filesystem_check.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-845fc211dd13f43969a9ff68"></a>
## with_dry_run

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::with_dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_dry_run(self, dry_run: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L126).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [196, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/filesystem_check.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only determine which add actions should be removed. A dry run will not commit actions to the Delta log

<a id="op-8a40b0eaed2a2d19053d1e01"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L51).

Source: `crates/core/src/operations/filesystem_check.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Commit properties and configuration

<a id="op-41df0fc15af72ae464593a26"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L52).

Source: `crates/core/src/operations/filesystem_check.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86ff5f62068b08d4f073ec77"></a>
## dry_run

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
dry_run: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L49).

Source: `crates/core/src/operations/filesystem_check.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Don't remove actions to the table log. Just determine which files can be removed

<a id="op-579376edf223dbc4dfcb3671"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [111, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/filesystem_check.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0950cfe8f000e74150c4fa6f"></a>
## log_store

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L105).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckBuilder", "path": "FileSystemCheckBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [111, 2], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/filesystem_check.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27ce201fb5e8461d5f3eb829"></a>
## log_store

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L47).

Source: `crates/core/src/operations/filesystem_check.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-5b646fec7bda18b53abf3246"></a>
## snapshot

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L45).

Source: `crates/core/src/operations/filesystem_check.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-checked table's state
