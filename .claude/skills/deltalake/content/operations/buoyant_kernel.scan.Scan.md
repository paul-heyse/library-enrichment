# `buoyant_kernel::scan::Scan`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.Scan.json).

<a id="op-34567ab1d979ca48ec5e8a90"></a>
## Scan

`struct` · `buoyant_kernel::scan::Scan` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Scan
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L650).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:650`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The result of building a scan over a table. This can be used to get the actual data from
scanning the table.

<a id="op-6dc0b3ab69a2f354c06c9103"></a>
## execute

`function` · `buoyant_kernel::scan::Scan::execute` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn execute(&self, engine: Arc<dyn Engine>) -> DeltaResult<impl Iterator<Item = DeltaResult<Box<dyn EngineData>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L1133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:1133`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Perform an "all in one" scan. This will use the provided `engine` to read and process all
the data for the query. Each [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) in the resultant iterator is a portion of the
final table data. Generally connectors/engines will want to use [`Scan::scan_metadata`](../operations/buoyant_kernel.scan.Scan.md#op-d91f731ebc1cdf3b0f6a19d8) so
they can have more control over the execution of the scan.

Returns an error if the scan was built with [`ScanBuilder::without_row_transforms`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-54f89c9f013e444f90b8d877); use
[`Scan::scan_metadata`](../operations/buoyant_kernel.scan.Scan.md#op-d91f731ebc1cdf3b0f6a19d8) instead.

<a id="op-7b2328d9212ccd1724ce6548"></a>
## fmt

`function` · `buoyant_kernel::scan::Scan::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L659).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [658, 1], "end": [667, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:659`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ba19820ffce66b9b38e59e3"></a>
## logical_schema

`function` · `buoyant_kernel::scan::Scan::logical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L712).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:712`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a shared reference to the logical [`Schema`] of the scan (i.e. the output schema of the
scan). Note that the logical schema can differ from the physical schema due to e.g.
partition columns which are present in the logical schema but not in the physical schema.

[`Schema`]: crate::schema::Schema

<a id="op-b7d12bb41b8bb2ae89677147"></a>
## parallel_scan_metadata

`function` · `buoyant_kernel::scan::Scan::parallel_scan_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parallel_scan_metadata(&self, engine: Arc<dyn Engine>) -> DeltaResult<SequentialScanMetadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L1091).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:1091`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Start a parallel scan metadata processing for the table.

This method returns a [`SequentialScanMetadata`](../operations/buoyant_kernel.parallel.parallel_scan_metadata.SequentialScanMetadata.md#op-05d3f38fd8bc5fd11a0e993a) iterator that processes commits and
checkpoint manifests sequentially. After exhausting this iterator, call `finish()`
to determine if a distributed phase is needed.

# Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::{Engine, DeltaResult};
# use delta_kernel::scan::{AfterSequentialScanMetadata, ParallelScanMetadata};
# use delta_kernel::Snapshot;
# use url::Url;
# use test_utils::delta_kernel_default_engine::DefaultEngineBuilder;
# use delta_kernel::object_store::local::LocalFileSystem;
# fn main() -> DeltaResult<()> {
let engine = Arc::new(DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new())).build());
let table_root = Url::parse("file:///path/to/table")?;

// Build a snapshot
let snapshot = Snapshot::builder_for(table_root.clone())
    .at_version(5) // Optional: specify a time-travel version (default is latest version)
    .build(engine.as_ref())?;
let scan = snapshot.scan_builder().build()?;
let mut sequential = scan.parallel_scan_metadata(engine.clone())?;

// Process sequential phase
for result in sequential.by_ref() {
    let scan_metadata = result?;
    // Process scan metadata...
}

// Check if distributed phase is needed
match sequential.finish()? {
    AfterSequentialScanMetadata::Done => {
        // All processing complete
    }
    AfterSequentialScanMetadata::Parallel { state, files } => {
        // Distribute files for parallel processing (e.g., one file per worker)
        let state = Arc::new(*state);
        for file in files {
            let parallel = ParallelScanMetadata::try_new(
                engine.clone(),
                state.clone(),
                vec![file],
            )?;
            for result in parallel {
                let scan_metadata = result?;
                // Process scan metadata...
            }
        }
    }
}
# Ok(())
# }

<a id="op-1ad1770ea401154260cf0cdf"></a>
## physical_predicate

`function` · `buoyant_kernel::scan::Scan::physical_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_predicate(&self) -> Option<PredicateRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L725).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:725`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the predicate [`PredicateRef`](../operations/buoyant_kernel.expressions.PredicateRef.md#op-70ac26119927823d3f871776) of the scan.

<a id="op-855132f2ee03be4526733b8b"></a>
## physical_schema

`function` · `buoyant_kernel::scan::Scan::physical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L720).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:720`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a shared reference to the physical [`Schema`] of the scan. This represents the schema
of the underlying data files which must be read from storage.

[`Schema`]: crate::schema::Schema

<a id="op-d91f731ebc1cdf3b0f6a19d8"></a>
## scan_metadata

`function` · `buoyant_kernel::scan::Scan::scan_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_metadata(&self, engine: &dyn Engine) -> DeltaResult<impl Iterator<Item = DeltaResult<ScanMetadata>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L758).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:758`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get an iterator of [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2)s that should be used to facilitate a scan. This handles
log-replay, reconciling Add and Remove actions, and applying data skipping (if possible).

Reports metrics: [`MetricEvent::ScanMetadataCompleted`] when the returned iterator is
fully exhausted.

[`MetricEvent::ScanMetadataCompleted`]: crate::metrics::MetricEvent::ScanMetadataCompleted

Each item in the returned iterator is a struct of:
- `Box<dyn EngineData>`: Data in engine format, where each row represents a file to be
  scanned. The schema for each row can be obtained by calling [`scan_row_schema`](../operations/buoyant_kernel.scan.scan_row_schema.md#op-e319333cbaa70c0f9bc8fb21).
- `Vec<bool>`: A selection vector. If a row is at index `i` and this vector is `false` at
  index `i`, then that row should *not* be processed (i.e. it is filtered out). If the
  vector is `true` at index `i` the row *should* be processed. If the selection vector is
  *shorter* than the number of rows returned, missing elements are considered `true`, i.e.
  included in the query. NB: If you are using the default engine and plan to call arrow's
  `filter_record_batch`, you _need_ to extend this vector to the full length of the batch or
  arrow will drop the extra rows.
- `Vec<Option<Expression>>`: Transformation expressions that need to be applied. For each
  row at index `i` in the above data, if an expression exists at index `i` in the `Vec`, the
  associated expression _must_ be applied to the data read from the file specified by the
  row. The resultant schema for this expression is guaranteed to be
  [`Self::logical_schema()`](../operations/buoyant_kernel.scan.Scan.md#op-2ba19820ffce66b9b38e59e3). If the item at index `i` in this `Vec` is `None`, or if the
  `Vec` contains fewer than `i` elements, no expression need be applied and the data read
  from disk is already in the correct logical state.

<a id="op-4351a2cb508c8a6ca9514c52"></a>
## scan_metadata_from

`function` · `buoyant_kernel::scan::Scan::scan_metadata_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_metadata_from(&self, engine: &dyn Engine, existing_version: Version, existing_data: impl IntoIterator<Item = Box<dyn EngineData>> + 'static, _existing_predicate: Option<PredicateRef>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<ScanMetadata>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L810).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:810`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get an updated iterator of [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2)s based on an existing iterator of
[`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809)s.

The existing iterator is assumed to contain data from a previous call to `scan_metadata`.
Engines may decide to cache the results of `scan_metadata` to avoid additional IO operations
required to replay the log.

As such the new scan's predicate must "contain" the previous scan's predicate. That is, the
new scan's predicate MUST skip all files the previous scan's predicate skipped. The new
scan's predicate is also allowed to skip files the previous predicate kept. For example,
if the previous scan predicate was
```sql
WHERE a < 42 AND b = 10
```
then it is legal for the new scan to use predicates such as the following:
```sql
WHERE a = 30 AND b = 10
WHERE a < 10 AND b = 10
WHERE a < 42 AND b = 10 AND c = 20
```
but it is NOT legal for the new scan to use predicates like these:
```sql
WHERE a < 42
WHERE a = 50 AND b = 10
WHERE a < 42 AND b <= 10
WHERE a < 42 OR b = 10
```

<div class="warning">

The current implementation does not yet validate the existing
predicate against the current predicate. Until this is implemented,
the caller must ensure that the existing predicate is compatible with
the current predicate.

</div>

# Parameters

* `existing_version` - Table version the provided data was read from.
* `existing_data` - Existing processed scan metadata with all selection vectors applied.
* `existing_predicate` - The predicate used by the previous scan.

<a id="op-26465eb99fc2d9c3158c7a7c"></a>
## snapshot

`function` · `buoyant_kernel::scan::Scan::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> &SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L703).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:703`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a shared reference to the [`Snapshot`] of this scan.

[`Snapshot`]: crate::Snapshot

<a id="op-f2c69e44b0836910b22d6800"></a>
## table_root

`function` · `buoyant_kernel::scan::Scan::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L696).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:696`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The table's root URL. Any relative paths returned from `scan_data` (or in a callback from
[`ScanMetadata::visit_scan_files`]) must be resolved against this root to get the actual
path to the file.

[`ScanMetadata::visit_scan_files`]: crate::scan::ScanMetadata::visit_scan_files

<a id="op-f153d5de84f3a1d3e02d690b"></a>
## correlation_id

`struct_field` · `buoyant_kernel::scan::Scan::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L654).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:654`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abd130520b78d2dffaa18383"></a>
## partition_values

`struct_field` · `buoyant_kernel::scan::Scan::partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values: PartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L655).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:655`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d3c37b5248f621e42fbfe88"></a>
## snapshot

`struct_field` · `buoyant_kernel::scan::Scan::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L651).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:651`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-076d70d28c6ac633054ba62a"></a>
## state_info

`struct_field` · `buoyant_kernel::scan::Scan::state_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state_info: std::sync::Arc<scan::state_info::StateInfo>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L652).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:652`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b35a85c316bdc2c6dad0fcd"></a>
## stats

`struct_field` · `buoyant_kernel::scan::Scan::stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats: StatsOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L653).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:653`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
