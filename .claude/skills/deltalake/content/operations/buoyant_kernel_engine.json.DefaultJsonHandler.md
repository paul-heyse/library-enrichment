# `buoyant_kernel_engine::json::DefaultJsonHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.json.DefaultJsonHandler.json).

<a id="op-11698a74ea845aaeab867667"></a>
## DefaultJsonHandler

`struct` · `buoyant_kernel_engine::json::DefaultJsonHandler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultJsonHandler<E: TaskExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02fabfcf69bbb081ede5d448"></a>
## fmt

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7050c961d66f0c6a4140302"></a>
## new

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L48).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-692663271af48f8de5d12b51"></a>
## parse_json

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::parse_json` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [193, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1288b0bd6690fcd7586da82d"></a>
## read_json_files

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::read_json_files` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [193, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-971a651d3abcafdef278c90f"></a>
## with_batch_size

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::with_batch_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L80).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Limit the number of rows per batch. That is, for batch_size = N, then each RecordBatch
yielded by the stream will have at most N rows.

Defaults to `super::DEFAULT_READ_BATCH_SIZE` rows (json objects).

See [Decoder::with_buffer_size] for details on constraining memory usage with buffer size
and batch size.

[Decoder::with_buffer_size]: delta_kernel::arrow::json::reader::Decoder

Unresolved upstream links (retained, not inferred): `delta_kernel::arrow::json::reader::Decoder`.

<a id="op-1f3a25d6892569a69d9c52cd"></a>
## with_buffer_size

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::with_buffer_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number read requests to buffer in memory at once in
[Self::read_json_files()](../operations/buoyant_kernel_engine.json.DefaultJsonHandler.md#op-1288b0bd6690fcd7586da82d).

Defaults to `super::DEFAULT_READ_BUFFER_SIZE`.

Memory constraints can be imposed by constraining the buffer size and batch size. Note that
overall memory usage is proportional to the product of these two values.
1. Batch size governs the size of RecordBatches yielded in each iteration of the stream
2. Buffer size governs the number of concurrent tasks (which equals the size of the buffer

<a id="op-206faa9ee099c288d1cf607b"></a>
## write_json_file

`function` · `buoyant_kernel_engine::json::DefaultJsonHandler::write_json_file` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_json_file(&self, path: &Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L180).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::json::DefaultJsonHandler", "path": "DefaultJsonHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [193, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:180`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-127f53e661f184956cd960c8"></a>
## batch_size

`struct_field` · `buoyant_kernel_engine::json::DefaultJsonHandler::batch_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
batch_size: std::num::NonZero<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L44).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Limit the number of rows per batch. That is, for batch_size = N, then each RecordBatch
yielded by the stream will have at most N rows.

<a id="op-b33a87b8fb25efd367dc07f6"></a>
## buffer_size

`struct_field` · `buoyant_kernel_engine::json::DefaultJsonHandler::buffer_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
buffer_size: std::num::NonZero<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L41).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The maximum number of read requests to buffer in memory at once. Note that this actually
controls two things: the number of concurrent requests (done by `buffered`) and the size of
the buffer (via our `sync_channel`).

<a id="op-d2c286eec332f8c567dbdb8e"></a>
## store

`struct_field` · `buoyant_kernel_engine::json::DefaultJsonHandler::store` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: std::sync::Arc<delta_kernel::object_store::DynObjectStore>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The object store to read files from

<a id="op-8da4fcb55785d5a52e1da493"></a>
## task_executor

`struct_field` · `buoyant_kernel_engine::json::DefaultJsonHandler::task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_executor: std::sync::Arc<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/json.rs#L37).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/json.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The executor to run async tasks on
