# `deltalake_core::kernel::snapshot::EagerSnapshot`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.EagerSnapshot.json).

<a id="op-523c35c411674d94607357a6"></a>
## EagerSnapshot

`struct` · `deltalake_core::kernel::snapshot::EagerSnapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1280).

Source: `crates/core/src/kernel/snapshot/mod.rs:1280`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of a Delta table that has been eagerly loaded into memory.

<a id="op-4ba3f6901ee49231205a3d8e"></a>
## add_actions_batches

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::add_actions_batches` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_actions_batches(&self, flatten: bool) -> Result<Vec<arrow::record_batch::RecordBatch>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L454).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [460, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:454`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Like [`add_actions_table`](Self::add_actions_table) but yields the actions as a sequence of
record batches instead of a single concatenated batch.

<a id="op-78fad59528fb6e5156f2b722"></a>
## add_actions_table

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::add_actions_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_actions_table(&self, flatten: bool) -> Result<arrow::record_batch::RecordBatch, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/state.rs#L445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [460, 2], "filename": "crates/core/src/table/state.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/state.rs:445`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Materialize the table's active `Add` actions as a single Arrow [`RecordBatch`].

When `flatten` is true, nested fields (e.g. partition values and statistics) are flattened
into top-level columns for easier inspection.

Unresolved upstream links (retained, not inferred): ``RecordBatch``.

<a id="op-ce5177a2492955c0390275d6"></a>
## arrow_schema

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::arrow_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_schema(&self) -> SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1432).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1432`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table arrow schema of the snapshot

<a id="op-dbc360dc1ea1e3bc97576181"></a>
## clone

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1279).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1279, 17], "end": [1279, 22], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1279`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d835c975b1e0bb57037cda0"></a>
## config

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn config(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L274).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [281, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:274`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b886eb7c49bbba31efc49f"></a>
## contained

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::contained` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contained(&self, column: &Column, value: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L219).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d7abc21c092b981a3d2d61b"></a>
## deserialize

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<D>(deserializer: D) -> Result<EagerSnapshot, D::Error> where D: Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/serde.rs#L514).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "super::EagerSnapshot"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [520, 2], "filename": "crates/core/src/kernel/snapshot/serde.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/snapshot/serde.rs:514`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9d6a06b30503f3f616e7a45"></a>
## domain_metadata

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::domain_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn domain_metadata(&self, log_store: &dyn LogStore, domain: impl ToString) -> DeltaResult<Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1521).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1521`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the configuration string stored for the given metadata `domain`, if present.

<a id="op-cd62a9a61432188c7498cabf"></a>
## eager_snapshot

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::eager_snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eager_snapshot(&self) -> &EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L278).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [281, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:278`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905aa23002562b18d7f1c854"></a>
## eq

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &EagerSnapshot) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1279).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1279, 24], "end": [1279, 33], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1279`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe43869b563f7ec3c8ee7d5a"></a>
## file_views

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::file_views` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_views(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1498).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1498`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream the active files in the snapshot

This function returns a stream of [`LogicalFileView`](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md#op-f5a43d85036c73510b6d2a44) objects,
which represent the active files in the snapshot.

## Parameters

* `log_store` - A reference to a [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementation.
* `predicate` - An optional predicate to filter the files.

## Returns

A stream of [`LogicalFileView`](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md#op-f5a43d85036c73510b6d2a44) objects, newest first.

<a id="op-b25b74f7973183262c35fe46"></a>
## fmt

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1279).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1279, 10], "end": [1279, 15], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1279`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5dc2b3735fc56abd7202bb4"></a>
## input_schema

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::input_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [228, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94f02587a2118670a10477cb"></a>
## load_config

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::load_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn load_config(&self) -> &DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1447).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1447`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table config which is loaded with of the snapshot

<a id="op-fe90eff6bb3be27c72d211f4"></a>
## log_data

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::log_data` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_data(&self) -> LogDataHandler<'_>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1475).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1475`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a [`LogDataHandler`](../operations/deltalake_core.kernel.snapshot.log_data.LogDataHandler.md#op-e8afbd2b8997b79f881a8888) for the snapshot to inspect the currently loaded state of the log.

<a id="op-b0c8c982d5d870a842ed86e6"></a>
## max_values

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::max_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L192).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:192`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the maximum values for the named column, if known.
Note: the returned array must contain `num_containers()` rows.

<a id="op-0d146dd426c026206abe7cb4"></a>
## metadata

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1437).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1437`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table metadata of the snapshot

<a id="op-5415b26a6898c78d7f04da01"></a>
## metadata

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L270).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [281, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:270`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6db21a180d250a86d3c988cd"></a>
## min_values

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::min_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L186).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:186`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the minimum values for the named column, if known.
Note: the returned array must contain `num_containers()` rows

<a id="op-8d5d226d129ffcdab33d0339"></a>
## null_counts

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::null_counts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of null values for the named column as an
`Option<UInt64Array>`.

Note: the returned array must contain `num_containers()` rows.

<a id="op-a3c8904db3ba21b368e46b58"></a>
## num_containers

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::num_containers` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_containers(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of containers (e.g. row groups) being
pruned with these statistics

<a id="op-b1410e0fe85acb1f6730b1a7"></a>
## parse_predicate_expression

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::parse_predicate_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [228, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2174d6a53483e7e6db6a57e4"></a>
## protocol

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L266).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [281, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::transaction::TableReference", "path": "TableReference"}, "trait_path": "deltalake_core::kernel::transaction::TableReference"}`

Source: `crates/core/src/kernel/transaction/mod.rs:266`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d0f56014823f5099964ce7"></a>
## protocol

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1442).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1442`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table protocol of the snapshot

<a id="op-90673319d7564cf63647af53"></a>
## read_schema

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::read_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L213).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [228, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-045d95e4ae5dee2fece8adcc"></a>
## row_counts

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::row_counts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/state.rs#L213).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "crate::kernel::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [222, 2], "filename": "crates/core/src/kernel/transaction/state.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/transaction/state.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of rows in each container as an `Option<UInt64Array>`.

Note: the returned array must contain `num_containers()` rows

<a id="op-a54ba6b748df07a48ac3d794"></a>
## schema

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> KernelSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1427`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table schema of the snapshot

<a id="op-777b9dfe90841ffb2c3b79b5"></a>
## serialize

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/serde.rs#L465).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "super::EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [464, 1], "end": [473, 2], "filename": "crates/core/src/kernel/snapshot/serde.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/snapshot/serde.rs:465`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f65793631c03b0d34fd25039"></a>
## table_configuration

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::table_configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_configuration(&self) -> &TableConfiguration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1457).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1457`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the resolved [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) (protocol, metadata and parsed properties).

<a id="op-9ab0b43ef7bd307a50bfef80"></a>
## table_properties

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::table_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1452).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1452`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Well known table configuration

<a id="op-3d21216ceac03842b7a8cbcd"></a>
## transaction_version

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::transaction_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn transaction_version(&self, log_store: &dyn LogStore, app_id: impl ToString) -> DeltaResult<Option<i64>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1510).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1510`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the latest committed transaction version for the given application id, if any.

This is used to implement idempotent writes: an application records its own monotonic
version via a transaction action, and readers can recover the last value seen.

<a id="op-1552531a108795ddccb11f18"></a>
## try_log_data

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::try_log_data` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_log_data(&self) -> DeltaResult<LogDataHandler<'_>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1463).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1463`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Try to get a [`LogDataHandler`](../operations/deltalake_core.kernel.snapshot.log_data.LogDataHandler.md#op-e8afbd2b8997b79f881a8888) for the snapshot to inspect the currently loaded state of
the log.

<a id="op-1cd5a520bea6fb2d3209ea24"></a>
## try_new

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1340).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1340`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`EagerSnapshot`](../operations/deltalake_core.kernel.snapshot.EagerSnapshot.md#op-523c35c411674d94607357a6) instance

<a id="op-65178f214105dc862dbf34b9"></a>
## version

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1406).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1406`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table version of the snapshot

<a id="op-ce0c95ac9159e4bb27ece6e1"></a>
## version_timestamp

`function` · `deltalake_core::kernel::snapshot::EagerSnapshot::version_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version_timestamp(&self, version: Version) -> Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1411).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 1], "end": [1528, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1411`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the timestamp of the given version

<a id="op-21d0264a49f8e73a575457a0"></a>
## snapshot

`struct_field` · `deltalake_core::kernel::snapshot::EagerSnapshot::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: std::sync::Arc<Snapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1281).

Source: `crates/core/src/kernel/snapshot/mod.rs:1281`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
