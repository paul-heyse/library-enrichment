# `deltalake_core::table::DeltaTable`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.DeltaTable.json).

<a id="op-2732e9061346704f1c6a0875"></a>
## DeltaTable

`struct` · `deltalake_core::table::DeltaTable` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L48).

Source: `crates/core/src/table/mod.rs:48`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

In memory representation of a Delta Table

A DeltaTable is a purely logical concept that represents a dataset that can evolve over time.
To attain concrete information about a table a snapshot need to be loaded.
Most commonly this is the latest state of the table, but may also loaded for a specific
version or point in time.

<a id="op-539d669fe294380510714bf9"></a>
## add_columns

`function` · `deltalake_core::table::DeltaTable::add_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_columns(self) -> AddColumnBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L165).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:165`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add new columns

<a id="op-8fb06cbe5cedf1769bc95b92"></a>
## add_constraint

`function` · `deltalake_core::table::DeltaTable::add_constraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_constraint(self) -> ConstraintBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L252).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:252`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add a check constraint to a table

<a id="op-a1986b8a1a4acb7259034938"></a>
## add_feature

`function` · `deltalake_core::table::DeltaTable::add_feature` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_feature(self) -> AddTableFeatureBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable a table feature for a table

<a id="op-8c0fb520c415265f342e8aad"></a>
## clone

`function` · `deltalake_core::table::DeltaTable::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaTable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L47).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "crates/core/src/table/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/mod.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99886f0c654fca14182c4a6c"></a>
## config

`struct_field` · `deltalake_core::table::DeltaTable::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
config: self::builder::DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L52).

Source: `crates/core/src/table/mod.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

the load options used during load

<a id="op-5e01948ef25b0e79f03a7296"></a>
## create

`function` · `deltalake_core::table::DeltaTable::create` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create(&self) -> CreateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new Delta table at this location, returning a [`CreateBuilder`](../operations/deltalake_core.operations.create.CreateBuilder.md#op-b50887bdd66591575052f481).

<a id="op-cc21a8977a893e16e9c6f12c"></a>
## delete

`function` · `deltalake_core::table::DeltaTable::delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete(self) -> DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L225).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:225`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete data from Delta table

<a id="op-5dccbdad758c5e2ec1ae25b1"></a>
## deserialize

`function` · `deltalake_core::table::DeltaTable::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [114, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/table/mod.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8828dd65a1b3b509772cecc0"></a>
## drop_column_not_null

`function` · `deltalake_core::table::DeltaTable::drop_column_not_null` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_column_not_null(self) -> DropColumnNotNullBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L177).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drop the `NOT NULL` constraint on a column, making it nullable

<a id="op-818ebf8ed873ec4d2a2ccab3"></a>
## drop_constraints

`function` · `deltalake_core::table::DeltaTable::drop_constraints` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_constraints(self) -> DropConstraintBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drops constraints from a table

<a id="op-1e795d1d301ce8c762e4bf1b"></a>
## filesystem_check

`function` · `deltalake_core::table::DeltaTable::filesystem_check` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn filesystem_check(self) -> FileSystemCheckBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L147).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:147`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Audit active files with files present on the filesystem

<a id="op-0f193004dac2dde9cc0ee232"></a>
## fmt

`function` · `deltalake_core::table::DeltaTable::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L488).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 1], "end": [492, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/table/mod.rs:488`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b7c30bdd70d870450f547ca"></a>
## fmt

`function` · `deltalake_core::table::DeltaTable::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L495).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [494, 1], "end": [498, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/mod.rs:495`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b85052aab7d7cc18ee006b9"></a>
## generate

`function` · `deltalake_core::table::DeltaTable::generate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn generate(self) -> GenerateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generate a symlink_format_manifest for other engines

<a id="op-2205ed586e7b64d06adf7edd"></a>
## get_active_add_actions_by_partitions

`function` · `deltalake_core::table::DeltaTable::get_active_add_actions_by_partitions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_active_add_actions_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> BoxStream<'_, DeltaResult<LogicalFileView>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L313).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:313`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream all logical files matching the provided conjunction (AND) of
`(column, op, value)` partition filter literals.

<a id="op-cbf335a41f96c766f31d40ea"></a>
## get_active_add_actions_by_predicate

`function` · `deltalake_core::table::DeltaTable::get_active_add_actions_by_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_active_add_actions_by_predicate(&self, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L299).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream all logical files matching the provided kernel [`Predicate`](delta_kernel::expressions::Predicate).

Predicates over partition columns select files exactly. Predicates over
data columns are evaluated against file statistics and select a superset:
every file that may contain a matching row, including files without
statistics for the referenced columns.

<a id="op-52226ae1718347de80deec3a"></a>
## get_file_uris

`function` · `deltalake_core::table::DeltaTable::get_file_uris` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_file_uris(&self) -> DeltaResult<impl Iterator<Item = String> + '_>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L367).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:367`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns a URIs for all active files present in the current table version.

<a id="op-88f097e5820785614557d8e2"></a>
## get_file_uris_by_partitions

`function` · `deltalake_core::table::DeltaTable::get_file_uris_by_partitions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_file_uris_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> Result<Vec<String>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L355).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:355`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the file uris as strings for the partition(s)

<a id="op-3c06b53ce88237aa694f0955"></a>
## get_files_by_partitions

`function` · `deltalake_core::table::DeltaTable::get_files_by_partitions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_files_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> Result<Vec<Path>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L341).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:341`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file list tracked in current table state filtered by provided
partition filter literals.

<a id="op-37a6a05c8eaa8acc02f44d3c"></a>
## get_latest_version

`function` · `deltalake_core::table::DeltaTable::get_latest_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_latest_version(&self) -> Result<Version, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L178).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:178`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

returns the latest available version of the table

<a id="op-80e9ed24a9f0a3008cd9a869"></a>
## history

`function` · `deltalake_core::table::DeltaTable::history` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn history(&self, limit: Option<usize>) -> Result<impl Iterator<Item = CommitInfo> + use<>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L267).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:267`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns provenance information, including the operation, user, and so on, for each write to a table.
The table history retention is based on the `logRetentionDuration` property of the Delta Table, 30 days by default.
If `limit` is given, this returns the information of the latest `limit` commits made to this table. Otherwise,
it returns all commits from the earliest commit.

<a id="op-c8ab34c263d11a6dad531707"></a>
## load

`function` · `deltalake_core::table::DeltaTable::load` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn load(&mut self) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L193).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:193`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Load DeltaTable with data from latest checkpoint

<a id="op-bd07223c753e46cf67ac2d8a"></a>
## load_version

`function` · `deltalake_core::table::DeltaTable::load_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn load_version(&mut self, version: Version) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:233`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Loads the DeltaTable state for the given version.

<a id="op-fbd40a4dc41d75936671eb6f"></a>
## load_with_datetime

`function` · `deltalake_core::table::DeltaTable::load_with_datetime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn load_with_datetime(&mut self, datetime: DateTime<Utc>) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L397).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:397`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time travel Delta table to the latest version that's created at or before provided
`datetime` argument.

Internally, this methods performs a binary search on all Delta transaction logs.

<a id="op-fae60d5534990fd3dd777f2a"></a>
## log_store

`function` · `deltalake_core::table::DeltaTable::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L173).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:173`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

get a shared reference to the log store

<a id="op-6520d30123b98065a7c363a4"></a>
## merge

`function` · `deltalake_core::table::DeltaTable::merge` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn merge<E: Into<Expression>>(self, source: datafusion::prelude::DataFrame, predicate: E) -> MergeBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L237).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:237`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update data from Delta table

<a id="op-297a8a569d2a5637ac2e7759"></a>
## new

`function` · `deltalake_core::table::DeltaTable::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(log_store: LogStoreRef, config: DeltaTableConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new Delta Table struct without loading any data from backing storage.

NOTE: This is for advanced users. If you don't know why you need to use this method, please
call one of the `open_table` helper methods instead.

<a id="op-3e75053902c2c2260987fc74"></a>
## new_in_memory

`function` · `deltalake_core::table::DeltaTable::new_in_memory` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_in_memory() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) instance, backed by an un-initialized in memory table

Using this will not persist any changes beyond the lifetime of the table object.
The main purpose of in-memory tables is for use in testing.

```
use deltalake_core::DeltaTable;
let table = DeltaTable::new_in_memory();
```

<a id="op-e5c3de0b31396efedadef69f"></a>
## object_store

`function` · `deltalake_core::table::DeltaTable::object_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store(&self) -> ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:158`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

get a shared reference to the delta object store

<a id="op-30b3d3701758d0059efa4146"></a>
## optimize

`function` · `deltalake_core::table::DeltaTable::optimize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn optimize<'a>(self) -> OptimizeBuilder<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L219).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Audit active files with files present on the filesystem

<a id="op-ccc2e63ad429a29aab780d31"></a>
## restore

`function` · `deltalake_core::table::DeltaTable::restore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn restore(self) -> RestoreBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Restore the table to an earlier version or timestamp, returning a [`RestoreBuilder`](../operations/deltalake_core.operations.restore.RestoreBuilder.md#op-3e336843be6cb6c5579089a2).

<a id="op-27518198c8dfbfd4c30b341f"></a>
## scan_cdf

`function` · `deltalake_core::table::DeltaTable::scan_cdf` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_cdf(self) -> CdfLoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Load a table with CDF Enabled

<a id="op-d8d9f3a456c209a46d18989d"></a>
## scan_table

`function` · `deltalake_core::table::DeltaTable::scan_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_table(&self) -> LoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read the table's data into Arrow record batches, returning a [`LoadBuilder`](../operations/deltalake_core.operations.load.LoadBuilder.md#op-55c2e757d2535f4cf337d261).

<a id="op-e416842b6632f29d3ddc557b"></a>
## serialize

`function` · `deltalake_core::table::DeltaTable::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [68, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/table/mod.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f09b7444e5abf18764595d49"></a>
## set_tbl_properties

`function` · `deltalake_core::table::DeltaTable::set_tbl_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set_tbl_properties(self) -> SetTablePropertiesBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:159`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set table properties

<a id="op-c3670c9649d61a8c124dacef"></a>
## snapshot

`function` · `deltalake_core::table::DeltaTable::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> DeltaResult<&DeltaTableState>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L389).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:389`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the currently loaded state snapshot.

This method provides access to the currently loaded state of the Delta table.

## Returns

A reference to the current state of the Delta table.

## Errors

Returns [`NotInitialized`](DeltaTableError::NotInitialized) if the table has not been initialized.

<a id="op-ec6f27f06dbb9c1558c64367"></a>
## state

`struct_field` · `deltalake_core::table::DeltaTable::state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
state: Option<self::state::DeltaTableState>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L50).

Source: `crates/core/src/table/mod.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The state of the table as of the most recent loaded Delta log entry.

<a id="op-e4d0e5daa6b84019197e3110"></a>
## table_provider

`function` · `deltalake_core::table::DeltaTable::table_provider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_provider(&self) -> TableProviderBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L471).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [467, 1], "end": [507, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:471`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a table provider for the table referenced by this DeltaTable.

See [`TableProviderBuilder`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-036ab68c33a0ecc37dd4caa6) for options when building the provider.

<a id="op-4e8c1ed0bb2b41127bee620c"></a>
## table_url

`function` · `deltalake_core::table::DeltaTable::table_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_url(&self) -> &Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L168).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:168`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The URI of the underlying data

<a id="op-f74e66012b666ef0fd0eb264"></a>
## try_from_url

`function` · `deltalake_core::table::DeltaTable::try_from_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_from_url(uri: Url) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L95).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) instance from a URL.

```
use deltalake_core::DeltaTable;
use url::Url;

async {
    let url = Url::parse("memory:///").unwrap();
    let ops = DeltaTable::try_from_url(url).await.unwrap();
};
```

<a id="op-3a37252639538f3030cca55b"></a>
## try_from_url_with_storage_options

`function` · `deltalake_core::table::DeltaTable::try_from_url_with_storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_from_url_with_storage_options(uri: Url, storage_options: HashMap<String, String>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) instance from URL with storage options

<a id="op-4da0a0df2c2c4c5389f518ca"></a>
## update

`function` · `deltalake_core::table::DeltaTable::update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update(self) -> UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L231).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:231`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update data from Delta table

<a id="op-d515f916d46cdb02f3dd6477"></a>
## update_datafusion_session

`function` · `deltalake_core::table::DeltaTable::update_datafusion_session` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update_datafusion_session(&self, session: &dyn Session) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L500).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [467, 1], "end": [507, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:500`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Ensure the provided DataFusion session is prepared to read this table.

This registers the table's root object store with the session's `RuntimeEnv` if missing.
Registration is idempotent and will not overwrite an existing mapping.

If the session already has an object store registered for the table's URL but it is stale or
incorrect, this method will not replace it. To override an existing mapping, call
`RuntimeEnv::register_object_store` directly.

```rust,no_run
use datafusion::prelude::SessionContext;
use deltalake_core::{DeltaResult, DeltaTable};

# fn main() -> DeltaResult<()> {
let table = DeltaTable::new_in_memory();
let ctx = SessionContext::new();
let state = ctx.state();
table.update_datafusion_session(&state)?;
# Ok(())
# }
```

<a id="op-b48003dd849211a9b73db4e3"></a>
## update_field_metadata

`function` · `deltalake_core::table::DeltaTable::update_field_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update_field_metadata(self) -> UpdateFieldMetadataBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update field metadata

<a id="op-47786db9820d24f711de5e6f"></a>
## update_incremental

`function` · `deltalake_core::table::DeltaTable::update_incremental` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn update_incremental(&mut self, max_version: Option<Version>) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L207).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:207`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Updates the DeltaTable by incrementally applying newer versions, optionally bounded by
`max_version`.

This API is forward-only. Use [`DeltaTable::load_version`](../operations/deltalake_core.table.DeltaTable.md#op-bd07223c753e46cf67ac2d8a) to load an older version.

<a id="op-802fb7a74a662eb222abfe68"></a>
## update_state

`function` · `deltalake_core::table::DeltaTable::update_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn update_state(&mut self) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Updates the DeltaTable to the most recent state committed to the transaction log by
loading the last checkpoint and incrementally applying each version since.

<a id="op-2ccdf99c4d35350d406fdb66"></a>
## update_table_metadata

`function` · `deltalake_core::table::DeltaTable::update_table_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update_table_metadata(self) -> UpdateTableMetadataBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L183).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:183`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update table metadata

<a id="op-f35c76fceb4dd387be47d4c5"></a>
## vacuum

`function` · `deltalake_core::table::DeltaTable::vacuum` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn vacuum(self) -> VacuumBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [191, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Vacuum stale files from delta table

<a id="op-1d5c5b021e590d55012ee569"></a>
## verify_deltatable_existence

`function` · `deltalake_core::table::DeltaTable::verify_deltatable_existence` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn verify_deltatable_existence(&self) -> DeltaResult<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if the [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) exists

<a id="op-8eddda9f3bbfadb225a1d010"></a>
## version

`function` · `deltalake_core::table::DeltaTable::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Option<Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [485, 2], "filename": "crates/core/src/table/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Currently loaded version of the table - if any.

This will return the latest version of the table if it has been loaded.
Returns `None` if the table has not been loaded.

<a id="op-8c868b08ecd74ae0be1525f8"></a>
## write

`function` · `deltalake_core::table::DeltaTable::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write(self, batches: impl IntoIterator<Item = RecordBatch>) -> WriteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L212).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::DeltaTable", "path": "crate::DeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [261, 2], "filename": "crates/core/src/operations/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/mod.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write the given record batches to the table, returning a [`WriteBuilder`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-fa55ac4ec569efc5b5c82899).

<a id="op-3b0dd1f7e5e5252cb20f4020"></a>
## log_store

`struct_field` · `deltalake_core::table::DeltaTable::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L54).

Source: `crates/core/src/table/mod.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

log store
