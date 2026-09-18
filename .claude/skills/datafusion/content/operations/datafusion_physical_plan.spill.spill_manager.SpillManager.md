# `datafusion_physical_plan::spill::spill_manager::SpillManager`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_manager.SpillManager.json).

<a id="op-06ac2cbfd91055e3e9c7b5c9"></a>
## SpillManager

`struct` · `datafusion_physical_plan::spill::spill_manager::SpillManager` · datafusion-physical-plan 55.1.0

```rust
struct SpillManager
```

Source: `src/spill/spill_manager.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The `SpillManager` is responsible for the following tasks:
- Reading and writing `RecordBatch`es to raw files based on the provided configurations.
- Updating the associated metrics.

Note: The caller (external operators such as `SortExec`) is responsible for interpreting the spilled files.
For example, all records within the same spill file are ordered according to a specific order.

<a id="op-2ac725819c17a33ae3011e39"></a>
## clone

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SpillManager
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/spill/spill_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/spill/spill_manager.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-920e8e27a4327c8aadfe37e8"></a>
## create_in_progress_file

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::create_in_progress_file` · datafusion-physical-plan 55.1.0

```rust
fn create_in_progress_file(&self, request_msg: &str) -> Result<InProgressSpillFile>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a temporary file for in-progress operations, returning an error
message if file creation fails. The file can be used to append batches
incrementally and then finish the file when done.

<a id="op-2bebf6837012c715c9f1803c"></a>
## fmt

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/spill/spill_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/spill/spill_manager.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdc1e6191e3462109ce49260"></a>
## new

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::new` · datafusion-physical-plan 55.1.0

```rust
fn new(env: Arc<RuntimeEnv>, metrics: SpillMetrics, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18e05fa7a1bcf0f1696210e3"></a>
## read_spill_as_stream

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::read_spill_as_stream` · datafusion-physical-plan 55.1.0

```rust
fn read_spill_as_stream(&self, spill_file_path: Arc<dyn SpillFile>, max_record_batch_memory: Option<usize>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reads a spill file as a stream. The file must be created by the current
`SpillManager`; otherwise an error will be returned.

Output is produced in FIFO order: the batch appended first is read first.

# Arg `max_record_batch_memory`

Most callers should pass `None`. This is mainly useful for the
memory-limited sort-preserving merge path.

When provided, this value is used only as a validation hint. If a
decoded batch exceeds this threshold, a debug-level log message is
emitted.

That path uses the maximum spilled batch size to conservatively estimate
the merge degree when merging multiple sorted runs.

<a id="op-55efc9c9884d2214d3c8dfb0"></a>
## read_spill_as_stream_unbuffered

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::read_spill_as_stream_unbuffered` · datafusion-physical-plan 55.1.0

```rust
fn read_spill_as_stream_unbuffered(&self, spill_file_path: Arc<dyn SpillFile>, max_record_batch_memory: Option<usize>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Same as `read_spill_as_stream`, but without buffering.

<a id="op-acee4a0453c8275131b66db0"></a>
## schema

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the schema for batches managed by this SpillManager

<a id="op-89a04337feed1c0a4fc1f9ac"></a>
## spill_record_batch_and_finish

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::spill_record_batch_and_finish` · datafusion-physical-plan 55.1.0

```rust
fn spill_record_batch_and_finish(&self, batches: &[RecordBatch], request_msg: &str) -> Result<Option<Arc<dyn SpillFile>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Spill input `batches` into a single file in a atomic operation. If it is
intended to incrementally write in-memory batches into the same spill file,
use [`Self::create_in_progress_file`](../operations/datafusion_physical_plan.spill.spill_manager.SpillManager.md#op-920e8e27a4327c8aadfe37e8) instead.
None is returned if no batches are spilled.

# Errors
- Returns an error if spilling would exceed the disk usage limit configured
  by `max_temp_directory_size` in `DiskManager`

<a id="op-029ab4825fb7d6ba7124eda3"></a>
## with_batch_read_buffer_capacity

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::with_batch_read_buffer_capacity` · datafusion-physical-plan 55.1.0

```rust
fn with_batch_read_buffer_capacity(self, batch_read_buffer_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7afc5ee1692455f01d236111"></a>
## with_compression_type

`function` · `datafusion_physical_plan::spill::spill_manager::SpillManager::with_compression_type` · datafusion-physical-plan 55.1.0

```rust
fn with_compression_type(self, spill_compression: SpillCompression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_manager::SpillManager", "path": "SpillManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [209, 2], "filename": "src/spill/spill_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_manager.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
