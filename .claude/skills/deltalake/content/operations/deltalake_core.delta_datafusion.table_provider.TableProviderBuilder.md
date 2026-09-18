# `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.json).

<a id="op-036ab68c33a0ecc37dd4caa6"></a>
## TableProviderBuilder

`struct` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableProviderBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L245).

Source: `crates/core/src/delta_datafusion/table_provider.rs:245`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for a datafusion [TableProvider] for a Delta table

A table provider can be built by providing either a log store, a Snapshot,
or an eager snapshot. If some Snapshot is provided, that will be used directly,
and no IO will be performed when building the provider.

Unresolved upstream links (retained, not inferred): `TableProvider`.

<a id="op-04f4c820ea479467e945cd39"></a>
## IntoFuture

`assoc_type` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <TableProviderBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L459).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [465, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:459`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c281d39cb0a8cd338ebf0ea"></a>
## Output

`assoc_type` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<Arc<dyn TableProvider>, DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L458).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [465, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:458`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf3a059c102915e30855e6d8"></a>
## build

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn build(self) -> Result<next::DeltaScan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L377).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:377`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Consume the builder and resolve it into an executable [`next::DeltaScan`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69).

<a id="op-3a8fa68915ad3a25451bb74a"></a>
## default

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L273).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [276, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:273`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c3da5a6327ad370ffc4d9bf"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [270, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd08e523ed3c6c459502fbd5"></a>
## into_future

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L461).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [465, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:461`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-563e53fb2a3e4002d6f51def"></a>
## with_adds

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_adds` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_adds(self, adds: impl IntoIterator<Item = Add>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L358).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:358`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to Add action paths. File metadata comes from the selected snapshot.

<a id="op-4748aa270b7f225c602d4aa7"></a>
## with_eager_snapshot

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_eager_snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_eager_snapshot(self, snapshot: impl Into<Arc<EagerSnapshot>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L299).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide an eager snapshot to use for the table provider

<a id="op-c03d3588a9e21396d3ce5ca5"></a>
## with_file_column

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_file_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_column(self, file_column: impl ToString) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L329).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:329`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the name of the file column to include in the scan

If specified, this will append a column to the table,
containing the source file path for each record.

<a id="op-bcbbe854c8264a166235a163"></a>
## with_file_paths

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_file_paths` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_paths(self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L363).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:363`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to file paths.

<a id="op-b76c954a8800fb3017ed40ed"></a>
## with_file_selection

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_file_selection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_selection(self, selection: next::FileSelection) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L371).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:371`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to a file selection resolved against the selected snapshot.

Path validation runs during scan or deletion vector planning. Malformed paths, paths
outside the table root, and missing selected files are reported there.

<a id="op-a601c3a5f6cccac8fc702820"></a>
## with_log_store

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_log_store(self, log_store: impl Into<Arc<dyn LogStore>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L293).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:293`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide the log store to use for the table provider

<a id="op-0369e5839026aab638b06488"></a>
## with_session

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_session` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session<S>(self, session: Arc<S>) -> Self where S: Session + 'static
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L311).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a DataFusion session for scan config defaults.

<a id="op-f92a41e6af0c3a1e071e2f89"></a>
## with_snapshot

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_snapshot(self, snapshot: impl Into<Arc<Snapshot>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L305).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a snapshot to use for the table provider

<a id="op-871e0b2228633a876325f515"></a>
## with_table_version

`function` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_table_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_version(self, version: impl Into<Option<Version>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L320).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::TableProviderBuilder", "path": "TableProviderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [455, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:320`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

**Reference annotation (source_runtime_reconciliation, separate from upstream):** A snapshot already present in the builder takes precedence in the retained fixture; select the table version before constructing from a loaded snapshot. [Evidence](../capabilities/delta.open.md).

Specify the version of the table to provide

<a id="op-2cf10d8893e9e4a34b2a7685"></a>
## file_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::file_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_column: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L249).

Source: `crates/core/src/delta_datafusion/table_provider.rs:249`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8aaa10a75b391293b369416"></a>
## file_selection

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::file_selection` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_selection: Option<next::FileSelection>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L254).

Source: `crates/core/src/delta_datafusion/table_provider.rs:254`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-730431a787eb0ee91ef66caf"></a>
## file_skipping_predicates

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::file_skipping_predicates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_skipping_predicates: Option<Vec<datafusion::prelude::Expr>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L253).

Source: `crates/core/src/delta_datafusion/table_provider.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Predicates used only for file skipping in kernel log replay

<a id="op-bc2c9e1029cc9aceb0f4ae2f"></a>
## log_store

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: Option<std::sync::Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L246).

Source: `crates/core/src/delta_datafusion/table_provider.rs:246`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5ce652c1f4a6b6c7dd7eeb2"></a>
## row_index_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::row_index_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_index_column: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L250).

Source: `crates/core/src/delta_datafusion/table_provider.rs:250`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4de269fae8c3dcd869fc3163"></a>
## session

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L248).

Source: `crates/core/src/delta_datafusion/table_provider.rs:248`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79a0c3ad0ea8cfbb9a29c8d1"></a>
## snapshot

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<delta_datafusion::table_provider::next::SnapshotWrapper>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L247).

Source: `crates/core/src/delta_datafusion/table_provider.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cdc8970d2d43143b710457b"></a>
## table_version

`struct_field` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::table_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_version: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L251).

Source: `crates/core/src/delta_datafusion/table_provider.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
