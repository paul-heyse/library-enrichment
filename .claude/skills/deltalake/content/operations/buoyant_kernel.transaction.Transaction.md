# `buoyant_kernel::transaction::Transaction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.Transaction.json).

<a id="op-fde417829ac3f4c320319f35"></a>
## Transaction

`struct` · `buoyant_kernel::transaction::Transaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Transaction<S = ExistingTable>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L198).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:198`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A transaction represents an in-progress write to a table. After creating a transaction, changes
to the table may be staged via the transaction methods before calling `commit` to commit the
changes to the table.

The type parameter `S` controls which operations are available:
- [`ExistingTable`](../operations/buoyant_kernel.transaction.ExistingTable.md#op-6ff6c69bae5eb319bf8e678e) (default): Full API for modifying existing tables.
- [`CreateTable`](../operations/buoyant_kernel.transaction.CreateTable.md#op-58c0a38333869df218438466): Restricted API for table creation (see
  [`CreateTableTransaction`](create_table::CreateTableTransaction)).

# Examples

```rust,ignore
// create a transaction
let mut txn = table.new_transaction(&engine)?;
// stage table changes (right now only commit info)
txn.commit_info(Box::new(ArrowEngineData::new(engine_commit_info)));
// commit! (consume the transaction)
txn.commit(&engine)?;
```

<a id="op-2684a1ae6ce0b1a22e5c2d44"></a>
## add_files

`function` · `buoyant_kernel::transaction::Transaction::add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_files(&mut self, add_metadata: Box<dyn EngineData>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1176`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Add files to include in this transaction. This API generally enables the engine to
add/append/insert data (files) to the table. Note that this API can be called multiple times
to add multiple batches.

The expected schema for `add_metadata` is given by [`Transaction::add_files_schema`](../operations/buoyant_kernel.transaction.Transaction.md#op-d8d74980cf5e0df7ce122a11).

<a id="op-d8d74980cf5e0df7ce122a11"></a>
## add_files_schema

`function` · `buoyant_kernel::transaction::Transaction::add_files_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_files_schema(&self) -> &'static SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L885).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:885`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The schema that the [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386)'s [`ParquetHandler`] is expected to use when reporting
information about a Parquet write operation back to Kernel.

Concretely, it is the expected schema for [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) passed to [`add_files`], as it is
the base for constructing an add_file. Each row represents metadata about a
file to be added to the table. Kernel takes this information and extends it to the full
add_file action schema, adding internal fields (e.g., baseRowID) as necessary.

The `stats` field contains file-level statistics. The schema returned here shows the base
structure; the actual stats written by `DefaultEngine::write_parquet` include dynamically
computed fields (numRecords, nullCount, minValues, maxValues, tightBounds) based on the
data schema and table configuration. See [`stats_schema`] for the table-specific expected
stats schema.

Note: While currently static, in the future the schema might change depending on
options set on the transaction or features enabled on the table.

[`add_files`]: crate::transaction::Transaction::add_files
[`ParquetHandler`]: crate::ParquetHandler
[`stats_schema`]: Transaction::stats_schema

<a id="op-5daba3b902f31af24aa6b69f"></a>
## commit

`function` · `buoyant_kernel::transaction::Transaction::commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit(self, engine: &dyn Engine) -> DeltaResult<CommitResult<S>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L349).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:349`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Consume the transaction and commit it to the table. The result is a result of
[CommitResult](../operations/buoyant_kernel.transaction.CommitResult.md#op-ef52c5b0205d38fde45a2f8f) with the following semantics:
- Ok(CommitResult) for either success or a recoverable error (includes the failed
  transaction in case of a conflict so the user can retry, etc.)
- Err(Error) indicates a non-retryable error (e.g. logic/validation error).

<a id="op-335ab608780bfbb4c6eb4558"></a>
## fmt

`function` · `buoyant_kernel::transaction::Transaction::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [269, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:258`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54f22cd1b305bf67a011c6ed"></a>
## logical_partition_columns

`function` · `buoyant_kernel::transaction::Transaction::logical_partition_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_partition_columns(&self) -> &[String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L988).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:988`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the logical partition column names for this table.

<a id="op-7c6776214df12f867382f5cb"></a>
## partitioned_write_context

`function` · `buoyant_kernel::transaction::Transaction::partitioned_write_context` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partitioned_write_context(&self, partition_values: HashMap<String, Scalar>) -> DeltaResult<WriteContext>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1102).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a write context for writing data to a specific partition.

Performs the following validations and transformations:

- **Key completeness**: ensures all partition columns are present and no extra keys exist.
  For example, if the table has partition columns `["year", "region"]` and you pass
  `{"year": Scalar::Integer(2024)}`, this returns an error for missing "region".

- **Case normalization**: matches keys case-insensitively against the schema and normalizes
  to schema case. For example, passing `"YEAR"` for a column named `"year"` is accepted and
  normalized.

- **Type checking**: rejects non-primitive partition column types (struct, array, map) and
  validates that each non-null `Scalar`'s type matches the partition column's schema type.
  For example, passing `Scalar::String("2024")` for an `INTEGER` column returns an error.
  Null-equivalent scalars (null scalars, empty strings, and empty binary) all of which
  collapse to JSON null in `partitionValues`) skip the value type check, but they are only
  legal when the partition column is nullable; passing any of these for a `nullable: false`
  partition column returns an error.

- **Value serialization**: serializes each `Scalar` to a protocol-compliant string per the
  Delta protocol's "Partition Value Serialization" rules. `Scalar::Null(...)` becomes `None`
  in `add.partitionValues` (JSON null). `Scalar::String("")` also becomes `None` (empty
  string equals null for all types). `Scalar::Date(19723)` becomes `Some("2024-01-01")`.

- **Key translation**: translates logical column names to physical names using the table's
  column mapping mode. For example, under `ColumnMappingMode::Name`, logical `"year"` might
  become physical `"col-abc-123"` in the `partitionValues` map.

- **Partition column materialization**: the returned [`WriteContext`](../operations/buoyant_kernel.transaction.write_context.WriteContext.md#op-dcfa01f4a2e15eb61c320d03)'s
  [`logical_to_physical`] expression injects partition columns when the table requires
  materializing partition columns (e.g. `materializePartitionColumns` or `icebergCompatV3`).
  The input data fed to that expression must not contain partition columns.

The returned [`WriteContext`](../operations/buoyant_kernel.transaction.write_context.WriteContext.md#op-dcfa01f4a2e15eb61c320d03) also provides a [`write_dir`] that returns the correct
target directory (Hive-style paths when column mapping is off, random prefix when on).

Returns an error if the table is not partitioned (use
[`unpartitioned_write_context`](Self::unpartitioned_write_context) instead).

[`write_dir`]: WriteContext::write_dir
[`logical_to_physical`]: WriteContext::logical_to_physical

<a id="op-021cead13e270102198f9940"></a>
## remove_files

`function` · `buoyant_kernel::transaction::Transaction::remove_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn remove_files(&mut self, remove_metadata: FilteredEngineData)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L179).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:179`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Remove files from the table in this transaction. This API generally enables the engine to
delete data (at file-level granularity) from the table. Note that this API can be called
multiple times to remove multiple batches.

The expected schema for `remove_metadata` is given by [`scan_row_schema`](../operations/buoyant_kernel.scan.scan_row_schema.md#op-e319333cbaa70c0f9bc8fb21). It is expected
this will be the result of passing [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) returned from a scan
with the selection vector modified to select rows for removal (selected rows in the
selection vector are the ones to be removed).

# Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::Engine;
# use delta_kernel::snapshot::Snapshot;
# use delta_kernel::committer::FileSystemCommitter;
# fn example(engine: Arc<dyn Engine>, table_url: url::Url) -> delta_kernel::DeltaResult<()> {
// Create a snapshot and transaction
let snapshot = Snapshot::builder_for(table_url).build(engine.as_ref())?;
let mut txn = snapshot.clone().transaction(Box::new(FileSystemCommitter::new()), engine.as_ref())?;

// Get file metadata from a scan
let scan = snapshot.scan_builder().build()?;
let scan_metadata = scan.scan_metadata(engine.as_ref())?;

// Remove specific files based on scan metadata
for metadata in scan_metadata {
    let metadata = metadata?;
    // In practice, you would modify the selection vector to choose which files to remove
    let files_to_remove = metadata.scan_files;
    txn.remove_files(files_to_remove);
}

// Commit the transaction
txn.commit(engine.as_ref())?;
# Ok(())
# }
```

<a id="op-a3801097623e8efb823e994e"></a>
## scan_metadata_to_engine_data

`function` · `buoyant_kernel::transaction::Transaction::scan_metadata_to_engine_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_metadata_to_engine_data(scan_metadata: impl Iterator<Item = DeltaResult<scan::ScanMetadata>>) -> impl Iterator<Item = DeltaResult<FilteredEngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L202).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:202`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Helper function to convert scan metadata iterator to filtered engine data iterator.

This adapter extracts the `scan_files` field from each [`crate::scan::ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) item,
making it easy to pass scan results directly to `update_deletion_vectors`.

# Example

```ignore
let scan = snapshot.scan_builder().build()?;
let metadata = scan.scan_metadata(engine)?;
let mut dv_map = HashMap::new();
// ... populate dv_map ...
let files_iter = Transaction::scan_metadata_to_engine_data(metadata);
txn.update_deletion_vectors(dv_map, files_iter)?;
```

<a id="op-92a21328cdde6db92a47bc66"></a>
## set_data_change

`function` · `buoyant_kernel::transaction::Transaction::set_data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set_data_change(&mut self, data_change: bool)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L592).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:592`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Same as [`Transaction::with_data_change`](../operations/buoyant_kernel.transaction.Transaction.md#op-683c80fff5a65100329dae31) but set the value directly instead of
using a fluent API.

<a id="op-b794f50568ee782c57fed57d"></a>
## stats_columns

`function` · `buoyant_kernel::transaction::Transaction::stats_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stats_columns(&self) -> Vec<ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L937).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:937`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the list of column names that should have statistics collected.

This returns leaf column paths as [`ColumnName`](../operations/buoyant_kernel.expressions.column_names.ColumnName.md#op-9a9657c136296c6d9c579ddd) objects. Each `ColumnName`
stores path components separately (e.g., `ColumnName::new(["nested", "field"])`).
See [`ColumnName`'s `Display` implementation][ColumnName#impl-Display-for-ColumnName](../operations/buoyant_kernel.expressions.column_names.ColumnName.md#op-9a9657c136296c6d9c579ddd)
for details on string formatting and escaping.

Engines can use this to determine which columns need stats during writes.

Per the Delta protocol, clustering columns are always included in statistics,
regardless of `dataSkippingStatsColumns` or `dataSkippingNumIndexedCols` settings.

<a id="op-c55def9c1eda7d94031b3aba"></a>
## stats_schema

`function` · `buoyant_kernel::transaction::Transaction::stats_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stats_schema(&self) -> DeltaResult<SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L918).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:918`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the expected schema for file statistics.

The schema structure is derived from table configuration:
- `delta.dataSkippingStatsColumns`: Explicit column list (if set)
- `delta.dataSkippingNumIndexedCols`: Column count limit (default 32)
- Partition columns: Always excluded

The returned schema has the following structure:
```ignore
{
  numRecords: long,
  nullCount: { ... },   // Nested struct mirroring data schema, all fields LONG
  minValues: { ... },   // Nested struct, only min/max eligible types
  maxValues: { ... },   // Nested struct, only min/max eligible types
  tightBounds: boolean,
}
```

Engines should collect statistics matching this schema structure when writing files.

Per the Delta protocol, required columns (e.g. clustering columns) are always included
in statistics, regardless of `dataSkippingStatsColumns` or `dataSkippingNumIndexedCols`
settings.

<a id="op-3ab7e517cb8e64e373004ac6"></a>
## unpartitioned_write_context

`function` · `buoyant_kernel::transaction::Transaction::unpartitioned_write_context` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unpartitioned_write_context(&self) -> DeltaResult<WriteContext>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1155).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::SupportsDataFiles", "path": "SupportsDataFiles"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 1], "end": [1179, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a write context for writing data to an unpartitioned table.

Returns an error if the table has partition columns (use
[`partitioned_write_context`](Self::partitioned_write_context) instead).

<a id="op-efc4e7dd2114282c97a7f24a"></a>
## update_deletion_vectors

`function` · `buoyant_kernel::transaction::Transaction::update_deletion_vectors` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update_deletion_vectors(&mut self, new_dv_descriptors: HashMap<String, DeletionVectorDescriptor>, existing_data_files: impl Iterator<Item = DeltaResult<FilteredEngineData>>) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L266).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:266`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Update deletion vectors for files in the table.

This method can be called multiple times to update deletion vectors for different sets of
files.

This method takes a map of file paths to new deletion vector descriptors and an iterator
of scan file data. It joins the two together internally and will generate appropriate
remove/add actions on commit to update the deletion vectors.

On commit, each matched file's add action carries `stats.tightBounds: false`.

# Arguments

* `new_dv_descriptors` - A map from data file path (as provided in scan operations) to the
  new deletion vector descriptor for that file.
* `existing_data_files` - An iterator over FilteredEngineData from scan metadata. The
  selected elements of each FilteredEngineData must be a superset of the paths that key
  `new_dv_descriptors`. Per the Delta protocol, files with deletion vectors must have an
  accurate `numRecords` statistic, so matched scan metadata must preserve that stat.

# Errors

Returns an error if:
- The transaction targets table creation instead of an existing table
- The table does not have deletion vectors enabled via protocol support and the
  `delta.enableDeletionVectors=true` table property
- A file path in `new_dv_descriptors` is not found in `existing_data_files`
- A matched file's scan metadata is missing or has invalid `stats.numRecords`
- A matched file's `stats` is not valid JSON or is not a JSON object

# Examples

```rust,ignore
let mut txn = snapshot.clone().transaction(Box::new(FileSystemCommitter::new()))?
    .with_operation("UPDATE".to_string());

let scan = snapshot.scan_builder().build()?;
let files: Vec<FilteredEngineData> = scan.scan_metadata(engine)?
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .map(|sm| sm.scan_files)
    .collect();

// Create map of file paths to new deletion vector descriptors
let mut dv_map = HashMap::new();
// ... populate dv_map with file paths and their new DV descriptors ...

txn.update_deletion_vectors(dv_map, files.into_iter())?;
txn.commit(engine)?;
```

<a id="op-84c6581013f3b3576ec5f563"></a>
## with_blind_append

`function` · `buoyant_kernel::transaction::Transaction::with_blind_append` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_blind_append(self) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Mark this transaction as a blind append.

Blind append transactions should only add new files and avoid write operations that
depend on existing table state.

<a id="op-4d6f06a1a638fa613b07c94a"></a>
## with_commit_info

`function` · `buoyant_kernel::transaction::Transaction::with_commit_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_info(self, engine_commit_info: Box<dyn EngineData>, commit_info_schema: SchemaRef) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L622).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:622`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the content of the commitInfo action for this transaction. Note that kernel will
_always_ write a commitInfo, this function simply allows engines to add their own data
into that action if they wish. Note that the following fields in `engine_commit_info`
will be overridden by kernel if they are set (meaning you should not set them):
- timestamp
- inCommitTimestamp
- operation
- operationParameters
- kernelVersion
- isBlindAppend
- engineInfo
- txnId

<a id="op-1ba379a9da956f60bff57d2b"></a>
## with_correlation_id

`function` · `buoyant_kernel::transaction::Transaction::with_correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L605).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:605`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attach an opaque, caller-supplied correlation id for joining this transaction's commit
metric events to the caller's own request or operation id. An empty id is treated as unset.
When unset, behavior is unchanged.

<a id="op-683c80fff5a65100329dae31"></a>
## with_data_change

`function` · `buoyant_kernel::transaction::Transaction::with_data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_data_change(self, data_change: bool) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L583).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:583`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the data change flag.

True indicates this commit is a "data changing" commit. False indicates table data was
reorganized but not materially modified.

Data change might be set to false in the following scenarios:
1. Operations that only change metadata (e.g. backfilling statistics)
2. Operations that make no logical changes to the contents of the table (i.e. rows are only
   moved from old files to new ones.  OPTIMIZE commands is one example of this type of
   optimizaton).

<a id="op-3e94abb34cdb07233a6d6540"></a>
## with_domain_metadata

`function` · `buoyant_kernel::transaction::Transaction::with_domain_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_domain_metadata(self, domain: String, configuration: String) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L648).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:648`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set domain metadata to be written to the Delta log.
Note that each domain can only appear once per transaction. That is, multiple configurations
of the same domain are disallowed in a single transaction, as well as setting and removing
the same domain in a single transaction. If a duplicate domain is included, the commit will
fail (that is, we don't eagerly check domain validity here).
Setting metadata for multiple distinct domains is allowed.

<a id="op-5acacb741b950d3425266e4b"></a>
## with_domain_metadata_removed

`function` · `buoyant_kernel::transaction::Transaction::with_domain_metadata_removed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_domain_metadata_removed(self, domain: String) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L135).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Remove domain metadata from the Delta log.
If the domain exists in the Delta log, this creates a tombstone to logically delete
the domain. The tombstone preserves the previous configuration value.
If the domain does not exist in the Delta log, this is a no-op.
Note that each domain can only appear once per transaction. That is, multiple operations
on the same domain are disallowed in a single transaction, as well as setting and removing
the same domain in a single transaction. If a duplicate domain is included, the `commit`
will fail (that is, we don't eagerly check domain validity here).
Removing metadata for multiple distinct domains is allowed.

<a id="op-c844378aad5d4e7e29fe6696"></a>
## with_engine_info

`function` · `buoyant_kernel::transaction::Transaction::with_engine_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_engine_info(self, engine_info: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L597).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:597`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the engine info field of this transaction's commit info action. This field is optional.

<a id="op-e63f702de25b0240138003d7"></a>
## with_operation

`function` · `buoyant_kernel::transaction::Transaction::with_operation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_operation(self, operation: String) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::Transaction", "path": "super::Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the operation that this transaction is performing. This string will be persisted in the
commit and visible to anyone who describes the table history.

<a id="op-77dbf9a67e6ba6e82926ca56"></a>
## with_transaction_id

`function` · `buoyant_kernel::transaction::Transaction::with_transaction_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_transaction_id(self, app_id: String, version: i64) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L636).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [888, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:636`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Include a SetTransaction (app_id and version) action for this transaction (with an optional
`last_updated` timestamp).
Note that each app_id can only appear once per transaction. That is, multiple app_ids with
different versions are disallowed in a single transaction. If a duplicate app_id is
included, the `commit` will fail (that is, we don't eagerly check app_id validity here).

<a id="op-c340e8dc39d3dbfe3ac3dd4f"></a>
## _state

`struct_field` · `buoyant_kernel::transaction::Transaction::_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_state: std::marker::PhantomData<S>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L254).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:254`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce594ff153f984f169ddd088"></a>
## add_files_metadata

`struct_field` · `buoyant_kernel::transaction::Transaction::add_files_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
add_files_metadata: Vec<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L220).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-145c4412a8451200bcf7791d"></a>
## commit_timestamp

`struct_field` · `buoyant_kernel::transaction::Transaction::commit_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_timestamp: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L230).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:230`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce37093597e55a6b036b6356"></a>
## committer

`struct_field` · `buoyant_kernel::transaction::Transaction::committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
committer: Box<dyn Committer>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L216).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:216`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a61c841889938865e1a1c76a"></a>
## correlation_id

`struct_field` · `buoyant_kernel::transaction::Transaction::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L204).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:204`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a490c7cf7b82a0e67706900"></a>
## data_change

`struct_field` · `buoyant_kernel::transaction::Transaction::data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L242).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75ef18beb01fc26d1d884f6d"></a>
## dv_matched_files

`struct_field` · `buoyant_kernel::transaction::Transaction::dv_matched_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
dv_matched_files: Vec<engine_data::FilteredEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L247).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:247`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bdf204ff028c8a28b3fdb39"></a>
## effective_table_config

`struct_field` · `buoyant_kernel::transaction::Transaction::effective_table_config` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
effective_table_config: table_configuration::TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L211).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:211`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbb5b78e2fc24970e3fb7770"></a>
## engine_commit_info

`struct_field` · `buoyant_kernel::transaction::Transaction::engine_commit_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
engine_commit_info: Option<(Box<dyn EngineData>, schema::SchemaRef)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L219).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:219`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1c213d01fb486f6a2bf0ca6"></a>
## engine_info

`struct_field` · `buoyant_kernel::transaction::Transaction::engine_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
engine_info: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L218).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bbd50e3cec32046f6ed37a5"></a>
## is_blind_append

`struct_field` · `buoyant_kernel::transaction::Transaction::is_blind_append` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
is_blind_append: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L244).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:244`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a2ab5efa02b8b30d1a018b7"></a>
## operation

`struct_field` · `buoyant_kernel::transaction::Transaction::operation` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L217).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:217`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40adb353e56fbfb10d04faae"></a>
## operation_id

`struct_field` · `buoyant_kernel::transaction::Transaction::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_id: metrics::MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L201).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:201`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffae2d914242c65a272db80b"></a>
## physical_clustering_columns

`struct_field` · `buoyant_kernel::transaction::Transaction::physical_clustering_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
physical_clustering_columns: Option<Vec<expressions::ColumnName>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L251).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f7eb1bb86cf9ef69e71a29d"></a>
## read_snapshot_opt

`struct_field` · `buoyant_kernel::transaction::Transaction::read_snapshot_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_snapshot_opt: Option<snapshot::SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L207).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:207`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9525788f675a9f3e5b8f1ad"></a>
## remove_files_metadata

`struct_field` · `buoyant_kernel::transaction::Transaction::remove_files_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
remove_files_metadata: Vec<engine_data::FilteredEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L221).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-484f60274695fa186b90fbea"></a>
## set_transactions

`struct_field` · `buoyant_kernel::transaction::Transaction::set_transactions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
set_transactions: Vec<actions::SetTransaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L227).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0c7fc134e48cc57dfb18eda"></a>
## should_emit_metadata

`struct_field` · `buoyant_kernel::transaction::Transaction::should_emit_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
should_emit_metadata: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L215).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:215`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-322d395edef69ba90a7b8f6c"></a>
## should_emit_protocol

`struct_field` · `buoyant_kernel::transaction::Transaction::should_emit_protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
should_emit_protocol: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L213).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:213`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d65e2646295cdcfb143bec10"></a>
## span

`struct_field` · `buoyant_kernel::transaction::Transaction::span` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
span: tracing::Span
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L199).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:199`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c876335884a4a43dc8fca3f3"></a>
## system_domain_metadata_additions

`struct_field` · `buoyant_kernel::transaction::Transaction::system_domain_metadata_additions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
system_domain_metadata_additions: Vec<actions::DomainMetadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L237).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:237`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20565719d4f8bf62f216687c"></a>
## user_domain_metadata_additions

`struct_field` · `buoyant_kernel::transaction::Transaction::user_domain_metadata_additions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
user_domain_metadata_additions: Vec<actions::DomainMetadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L232).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:232`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ff2a2baf6dbb79081db3bba"></a>
## user_domain_removals

`struct_field` · `buoyant_kernel::transaction::Transaction::user_domain_removals` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
user_domain_removals: Vec<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L240).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:240`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
