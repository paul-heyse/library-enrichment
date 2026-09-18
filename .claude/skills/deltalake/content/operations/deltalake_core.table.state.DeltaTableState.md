# `deltalake_core::table::state::DeltaTableState`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.state.DeltaTableState.json).

<a id="op-06565907dcae2ffe929df798"></a>
## DeltaTableState

`struct` · `deltalake_core::table::state::DeltaTableState` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTableState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L33).

Source: `crates/core/src/table/state.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

State snapshot currently held by the Delta Table instance.

<a id="op-6cabde008d1569cab93b157c"></a>
## add_actions_batches

`function` · `deltalake_core::table::state::DeltaTableState::add_actions_batches` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_actions_batches(&self, flatten: bool) -> Result<Vec<arrow::record_batch::RecordBatch>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L207).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:207`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get add action data as a list of [arrow::record_batch::RecordBatch]
without concatenating them into a single batch.

Unresolved upstream links (retained, not inferred): `arrow::record_batch::RecordBatch`.

<a id="op-29b30a4738e8bd97f18a37f3"></a>
## add_actions_table

`function` · `deltalake_core::table::state::DeltaTableState::add_actions_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_actions_table(&self, flatten: bool) -> Result<arrow::record_batch::RecordBatch, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get an [arrow::record_batch::RecordBatch] containing add action data.

# Arguments

* `flatten` - whether to flatten the schema. Partition values columns are
  given the prefix `partition.`, statistics (null_count, min, and max) are
  given the prefix `null_count.`, `min.`, and `max.`, and tags the
  prefix `tags.`. Nested field names are concatenated with `.`.

# Data schema

Each row represents a file that is a part of the selected tables state.

* `path` (String): relative or absolute to a file.
* `size_bytes` (Int64): size of file in bytes.
* `modification_time` (Millisecond Timestamp): time the file was created.
* `null_count.{col_name}` (Int64): number of null values for column in
  this file.
* `num_records.{col_name}` (Int64): number of records for column in
  this file.
* `min.{col_name}` (matches column type): minimum value of column in file
  (if available).
* `max.{col_name}` (matches column type): maximum value of column in file
  (if available).
* `partition.{partition column name}` (matches column type): value of
  partition the file corresponds to.

Unresolved upstream links (retained, not inferred): `arrow::record_batch::RecordBatch`.

<a id="op-5a2498c4e88e7091cbec0aed"></a>
## all_tombstones

`function` · `deltalake_core::table::state::DeltaTableState::all_tombstones` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn all_tombstones(&self, log_store: &dyn LogStore) -> BoxStream<'_, DeltaResult<TombstoneView>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Full list of tombstones (remove actions) representing files removed from table state).

<a id="op-870bab525ddcac0ac3d8ae15"></a>
## clone

`function` · `deltalake_core::table::state::DeltaTableState::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaTableState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "crates/core/src/table/state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/state.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a383f59dd2b1abaaea95aad"></a>
## config

`function` · `deltalake_core::table::state::DeltaTableState::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn config(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L284).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "crate::table::state::DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:284`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f6101884edf26c29b0b419a"></a>
## deserialize

`function` · `deltalake_core::table::state::DeltaTableState::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 35], "end": [31, 46], "filename": "crates/core/src/table/state.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/table/state.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daa6b4d49dafa44b55d2eb67"></a>
## eager_snapshot

`function` · `deltalake_core::table::state::DeltaTableState::eager_snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eager_snapshot(&self) -> &EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L296).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "crate::table::state::DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:296`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94c83823a03645e3d48daf5a"></a>
## fmt

`function` · `deltalake_core::table::state::DeltaTableState::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "crates/core/src/table/state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/state.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19fd34e1eaa7b4b0855d7232"></a>
## load_config

`function` · `deltalake_core::table::state::DeltaTableState::load_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn load_config(&self) -> &DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table config which is loaded with of the snapshot

<a id="op-ee7fabac3a512ec660219df1"></a>
## log_data

`function` · `deltalake_core::table::state::DeltaTableState::log_data` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_data(&self) -> LogDataHandler<'_>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns a semantic accessor to the currently loaded log data.

<a id="op-5a7bf4033dfbf7dcf15ec43a"></a>
## metadata

`function` · `deltalake_core::table::state::DeltaTableState::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "crate::table::state::DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:292`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ed54fd612298e0329ae5df4"></a>
## metadata

`function` · `deltalake_core::table::state::DeltaTableState::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The most recent metadata of the table.

<a id="op-8bd6a69d1010a8cf6957278e"></a>
## new

`function` · `deltalake_core::table::state::DeltaTableState::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(snapshot: EagerSnapshot) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wrap an already-loaded [`EagerSnapshot`](../operations/deltalake_core.kernel.snapshot.EagerSnapshot.md#op-523c35c411674d94607357a6) as the in-memory state of a table.

<a id="op-043dde8925a5ad38befb1326"></a>
## protocol

`function` · `deltalake_core::table::state::DeltaTableState::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L288).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "crate::table::state::DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:288`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14eb6483ba7fcaf3a22b1695"></a>
## protocol

`function` · `deltalake_core::table::state::DeltaTableState::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The most recent protocol of the table.

<a id="op-b95b44c472c63673509bf921"></a>
## schema

`function` · `deltalake_core::table::state::DeltaTableState::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> KernelSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The table schema

<a id="op-9aa607541bd28f98e2dacb53"></a>
## serialize

`function` · `deltalake_core::table::state::DeltaTableState::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 24], "end": [31, 33], "filename": "crates/core/src/table/state.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/table/state.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-320a27ff854d6d89dda4ab37"></a>
## snapshot

`function` · `deltalake_core::table::state::DeltaTableState::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> &EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L157).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Obtain the Eager snapshot of the state

<a id="op-6b2803b8b109a1129d942fb3"></a>
## table_config

`function` · `deltalake_core::table::state::DeltaTableState::table_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_config(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L81).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Well known table configuration

<a id="op-2f37fffdce7544279b1819db"></a>
## transaction_version

`function` · `deltalake_core::table::state::DeltaTableState::transaction_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn transaction_version(&self, log_store: &dyn LogStore, app_id: impl ToString) -> DeltaResult<Option<i64>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the transaction version for the given application ID.

Returns `None` if the application ID is not found.

<a id="op-21543194dd05425239c65c67"></a>
## try_new

`function` · `deltalake_core::table::state::DeltaTableState::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new DeltaTableState

<a id="op-1250319bafe61629b3510089"></a>
## update

`function` · `deltalake_core::table::state::DeltaTableState::update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn update(&mut self, log_store: &dyn LogStore, version: Option<Version>) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:162`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update the state of the table to the given version.

<a id="op-d0cb5d8ec1e3dffa3a67df52"></a>
## version

`function` · `deltalake_core::table::state::DeltaTableState::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L56).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return table version

<a id="op-6327938011b0e7f05b28522e"></a>
## version_timestamp

`function` · `deltalake_core::table::state::DeltaTableState::version_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version_timestamp(&self, version: Version) -> Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::state::DeltaTableState", "path": "DeltaTableState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [213, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the timestamp when a version commit was created.
This is the timestamp of the commit file.
If the commit file is not present, None is returned.

<a id="op-39cfdc3b17439bc52041c8f0"></a>
## snapshot

`struct_field` · `deltalake_core::table::state::DeltaTableState::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: kernel::EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L34).

Source: `crates/core/src/table/state.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
