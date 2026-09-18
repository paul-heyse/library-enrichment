# `buoyant_kernel_engine::DefaultEngineBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.DefaultEngineBuilder.json).

<a id="op-d6de93f6e92c25ea359b026f"></a>
## DefaultEngineBuilder

`struct` · `buoyant_kernel_engine::DefaultEngineBuilder` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultEngineBuilder<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L129).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:129`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for creating [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5) instances.

# Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel_engine as delta_kernel_default_engine;
# use delta_kernel_default_engine::DefaultEngineBuilder;
# use delta_kernel_default_engine::executor::tokio::TokioBackgroundExecutor;
# use delta_kernel::object_store::local::LocalFileSystem;
// Build a DefaultEngine with default executor
let engine = DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new()))
    .build();

// Build with a custom executor
let engine = DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new()))
    .with_task_executor(Arc::new(TokioBackgroundExecutor::new()))
    .build();
```

<a id="op-65d1e79012c863bb6bd963c2"></a>
## build

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::build` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DefaultEngine<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L210).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "std::sync::Arc"}}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 1], "end": [213, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:210`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build the [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5) instance.

<a id="op-75ea9b7bcbbb13bc9ada7fa3"></a>
## build

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::build` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DefaultEngine<executor::tokio::TokioBackgroundExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L165).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::DefaultTaskExecutor", "path": "DefaultTaskExecutor"}}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [169, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:165`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build the [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5) instance.

<a id="op-dce137fa0e3c03eb16e3b3ce"></a>
## fmt

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 10], "end": [128, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e29a8ea4dd2612a48463760f"></a>
## new

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(object_store: Arc<DynObjectStore>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L156).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::DefaultTaskExecutor", "path": "DefaultTaskExecutor"}}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [169, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:156`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`DefaultEngineBuilder`](../operations/buoyant_kernel_engine.DefaultEngineBuilder.md#op-d6de93f6e92c25ea359b026f) instance with the default executor.

<a id="op-8aa6416d4ab961e7f50239b1"></a>
## with_batch_size

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::with_batch_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L202).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [206, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:202`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number of rows per batch yielded by the JSON and Parquet handlers in their
`read_*_files` paths.

Defaults to the handlers' built-in value when unset. Overall read memory usage is roughly
proportional to `buffer_size * batch_size`.

<a id="op-cd227ae3cd6f2e749d37b799"></a>
## with_buffer_size

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::with_buffer_size` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L192).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [206, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:192`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number of files read concurrently by the JSON and Parquet handlers in their
`read_*_files` paths. This is the file-level I/O readahead depth: higher values overlap more
object-store requests to hide latency, at the cost of more in-flight memory.

Defaults to the handlers' built-in value when unset. Ordering of returned data is preserved
regardless of this value.

<a id="op-5cef1ff78254d434e67ac04b"></a>
## with_task_executor

`function` · `buoyant_kernel_engine::DefaultEngineBuilder::with_task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_task_executor<F: TaskExecutor>(self, task_executor: Arc<F>) -> DefaultEngineBuilder<Arc<F>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L175).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngineBuilder", "path": "DefaultEngineBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [206, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set a custom task executor for the engine.

See [`executor::TaskExecutor`](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33) for more details.

<a id="op-5a4fe78371a8856da6fcb961"></a>
## io_config

`struct_field` · `buoyant_kernel_engine::DefaultEngineBuilder::io_config` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
io_config: ReadIoConfig
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L135).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read-path I/O concurrency config applied to the JSON and Parquet handlers. `None` fields
fall back to the handlers' defaults.

<a id="op-d976bc8d349e919a5bf8cef5"></a>
## object_store

`struct_field` · `buoyant_kernel_engine::DefaultEngineBuilder::object_store` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_store: std::sync::Arc<delta_kernel::object_store::DynObjectStore>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L130).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:130`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44f4c50def9e99aebb3de446"></a>
## task_executor

`struct_field` · `buoyant_kernel_engine::DefaultEngineBuilder::task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_executor: E
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L132).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:132`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The state is either [`DefaultTaskExecutor`](../operations/buoyant_kernel_engine.DefaultTaskExecutor.md#op-b61f2eab2b7d1ecee759d3fa) or `Arc<E>` with a custom task executor.
