# `buoyant_kernel_engine::parquet::DefaultParquetHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.parquet.DefaultParquetHandler.json).

<a id="op-e65eba96a1e47c1a6710e3b6"></a>
## DefaultParquetHandler

`struct` · `buoyant_kernel_engine::parquet::DefaultParquetHandler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultParquetHandler<E: TaskExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L46).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3baf0b120f233a30bf2939ee"></a>
## fmt

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L45).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-447307ec64909c9b2443de31"></a>
## new

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [266, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32424bc2219897d70c902178"></a>
## read_parquet_files

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::read_parquet_files` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L322).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [430, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:322`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c995c1199c6e3e3b66b3613"></a>
## read_parquet_footer

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::read_parquet_footer` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L401).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [430, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:401`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0990128598c1937326e16717"></a>
## with_batch_size

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::with_batch_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L183).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [266, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:183`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number of rows per RecordBatch yielded by [Self::read_parquet_files()](../operations/buoyant_kernel_engine.parquet.DefaultParquetHandler.md#op-32424bc2219897d70c902178).

Defaults to `super::DEFAULT_READ_BATCH_SIZE` rows.

<a id="op-57918498c489f79ed32761dc"></a>
## with_buffer_size

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::with_buffer_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L175).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [266, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number of files to read concurrently in [Self::read_parquet_files()](../operations/buoyant_kernel_engine.parquet.DefaultParquetHandler.md#op-32424bc2219897d70c902178).

Defaults to `super::DEFAULT_READ_BUFFER_SIZE`.

This setting applies only to object-store reads. When every file in the batch uses a
presigned URL (`https://...`), reads bypass the object store and this value has no effect;
use [`Self::with_batch_size`](../operations/buoyant_kernel_engine.parquet.DefaultParquetHandler.md#op-0990128598c1937326e16717) to tune RecordBatch chunking in that path.

Memory constraints can be imposed by constraining the buffer size and batch size. Note that
overall memory usage is proportional to the product of these two values.
1. Batch size governs the size of RecordBatches yielded in each iteration of the stream.
2. Buffer size governs the number of concurrent file reads (which equals the size of the
   readahead buffer).

<a id="op-ba4110f3c4715c03cb400603"></a>
## write_parquet_file

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::write_parquet_file` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L353).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [430, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:353`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Writes engine data to a Parquet file at the specified location.

This implementation uses asynchronous file I/O with object_store to write the Parquet file.
If a file already exists at the given location, it will be overwritten.

# Parameters

- `location` - The full URL path where the Parquet file should be written (e.g., `s3://bucket/path/file.parquet`,
  `file:///path/to/file.parquet`).
- `data` - An iterator of engine data to be written to the Parquet file.

# Returns

A [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) indicating success or failure.

<a id="op-cf5ceeb5cf7197d0680cbde0"></a>
## write_parquet_file

`function` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::write_parquet_file` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_parquet_file(&self, data: Box<dyn EngineData>, write_context: &WriteContext) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L252).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::parquet::DefaultParquetHandler", "path": "DefaultParquetHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [266, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:252`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write `data` to a new parquet file under the [`WriteContext::write_dir`] and return
Add action metadata ready for [`Transaction::add_files`].

Note that the schema does not contain the dataChange column. In order to set `data_change`
flag, use [`delta_kernel::transaction::Transaction::with_data_change`].

[`WriteContext::write_dir`]: delta_kernel::transaction::WriteContext::write_dir
[`Transaction::add_files`]: delta_kernel::transaction::Transaction::add_files

Unresolved upstream links (retained, not inferred): `delta_kernel::transaction::WriteContext::write_dir`, `delta_kernel::transaction::Transaction::add_files`, ``delta_kernel::transaction::Transaction::with_data_change``.

<a id="op-cc237333dc553b3d0c99c7d6"></a>
## batch_size

`struct_field` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::batch_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
batch_size: std::num::NonZero<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The maximum number of rows per RecordBatch yielded by the read stream.

<a id="op-e549eeaf02a4740948736901"></a>
## buffer_size

`struct_field` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::buffer_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
buffer_size: std::num::NonZero<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The maximum number of files to read concurrently in [`Self::read_parquet_files()`]. This is
the number of futures buffered by `buffered`, i.e. the file-level I/O readahead depth.

Unresolved upstream links (retained, not inferred): ``Self::read_parquet_files()``.

<a id="op-9a13c813a89d44577d03bb34"></a>
## store

`struct_field` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::store` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: std::sync::Arc<delta_kernel::object_store::DynObjectStore>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c104a376c4d7d7ef9420e47b"></a>
## task_executor

`struct_field` · `buoyant_kernel_engine::parquet::DefaultParquetHandler::task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_executor: std::sync::Arc<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/parquet.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/parquet.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
