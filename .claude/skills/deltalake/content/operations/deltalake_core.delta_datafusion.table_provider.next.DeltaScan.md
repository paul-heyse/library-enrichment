# `deltalake_core::delta_datafusion::table_provider::next::DeltaScan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.DeltaScan.json).

<a id="op-e35fcdd4ce5cf85731bf3f69"></a>
## DeltaScan

`struct` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaScan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L488).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:488`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An executable, serializable Delta table scan.

`DeltaScan` captures everything needed to read a consistent set of data files from a
table snapshot — the resolved schemas, optional file-skipping predicates, deletion-vector
aware file selection and the originating log store. It is the unit produced by
[`TableProviderBuilder`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-036ab68c33a0ecc37dd4caa6) and consumed by DataFusion's execution layer.

<a id="op-0ee66ec274d1598391208f9d"></a>
## builder

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::builder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder() -> TableProviderBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L705).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:705`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Start building a scan/table provider with a fluent [`TableProviderBuilder`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-036ab68c33a0ecc37dd4caa6).

<a id="op-6d3464903d668c9d4664d952"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaScan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L487).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 10], "end": [487, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:487`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1853b1a84f35623a74901b6e"></a>
## deletion_vectors

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::deletion_vectors` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn deletion_vectors(&self, session: &dyn Session) -> Result<Vec<DeletionVectorSelection>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L640).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:640`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Materialize deletion vector keep masks for files in this scan.

The result is sorted lexicographically by filepath for deterministic ordering and includes
only files that have deletion vectors.

This API materializes all deletion vectors in memory.

<a id="op-64d29b684df812f0a5786d0e"></a>
## deserialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L487).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 35], "end": [487, 46], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:487`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09240020495851183f220b3c"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L487).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 17], "end": [487, 22], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:487`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a2f63021ae6384e734d3a64"></a>
## get_logical_plan

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::get_logical_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L724).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:724`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4ee24f1ec0bf168fa1c72f8"></a>
## get_table_definition

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::get_table_definition` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_table_definition(&self) -> Option<&str>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L720).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:720`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bef232698ee619f91cfb9f5"></a>
## insert_into

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::insert_into` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn insert_into(&self, state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L770).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:770`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-734baf796342421fbc9697db"></a>
## new

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(snapshot: impl Into<SnapshotWrapper>, config: DeltaScanConfig) -> Result<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L518).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:518`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new scan over the given `snapshot` using `config`.

Validates that the snapshot only uses reader features delta-rs supports and resolves
the scan and provider (public) schemas, including the optional file-id column.

<a id="op-01f96c7a0b590714fbd64ec4"></a>
## scan

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L728).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:728`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9da076d24ee413cc16d2aed3"></a>
## schema

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L712).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:712`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a24b7b4a6c6f97ada2102b41"></a>
## serialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L487).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 24], "end": [487, 33], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:487`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73f2ee5e30aee5916e2c963c"></a>
## supports_filters_pushdown

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::supports_filters_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L810).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:810`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2c09b53a1a7f0363d62b61e"></a>
## table_type

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::table_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_type(&self) -> TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L716).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [820, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:716`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca70980fe3a338338fdd7777"></a>
## with_adds

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::with_adds` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_adds(self, adds: impl IntoIterator<Item = Add>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L556).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:556`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to Add action paths. File metadata comes from the scan snapshot.

<a id="op-de63b39ed05c8635ce2075fe"></a>
## with_file_paths

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::with_file_paths` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_paths(self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L561).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:561`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to file paths.

<a id="op-49312fb115d78ba665d85db5"></a>
## with_file_selection

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::with_file_selection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_selection(self, selection: FileSelection) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L569).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeltaScan", "path": "DeltaScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [708, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:569`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restrict reads to a file selection resolved against this scan's snapshot.

Path validation runs during scan or deletion vector planning. Malformed paths, paths
outside the table root, and missing selected files are reported there.

<a id="op-a6fdff4b780ab9c973b0d7a1"></a>
## config

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: delta_datafusion::DeltaScanConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L490).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:490`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50397850b3236fd1811e1f4e"></a>
## file_selection

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::file_selection` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_selection: Option<FileSelection>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L501).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:501`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeeb4c16a83ed9bc5b99736c"></a>
## file_skipping_predicate

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::file_skipping_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_skipping_predicate: Option<Vec<datafusion::prelude::Expr>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L496).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:496`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e16d9b34d0dd2aebef93cc8e"></a>
## full_schema

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::full_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
full_schema: arrow::datatypes::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L493).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:493`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provider/public schema, including configured file id capability when enabled.

<a id="op-2c08d3d5a75fd78c7a867eec"></a>
## log_store

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: Option<logstore::LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L498).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:498`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc44fd1acc934db25279147d"></a>
## read_operation_id

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::read_operation_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_operation_id: Option<uuid::Uuid>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L500).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:500`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6607d27c5979b600fe09b6e"></a>
## row_index_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::row_index_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_index_column: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L494).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:494`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-083e10008514e9205b8ef472"></a>
## scan_schema

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::scan_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
scan_schema: arrow::datatypes::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L491).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:491`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19b212b4a2f5d961ebc17f68"></a>
## snapshot

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: SnapshotWrapper
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L489).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:489`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
