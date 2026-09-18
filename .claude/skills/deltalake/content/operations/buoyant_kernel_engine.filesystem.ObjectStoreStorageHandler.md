# `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.filesystem.ObjectStoreStorageHandler.json).

<a id="op-4af7ed40425a88b6c5da8ab8"></a>
## ObjectStoreStorageHandler

`struct` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ObjectStoreStorageHandler<E: TaskExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L15).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689dd905ec6642d435732c8b"></a>
## copy_atomic

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::copy_atomic` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e0db71f9e624fe129aaae47"></a>
## delete

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::delete` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete(&self, path: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:233`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be81e271e9c4426eed6dca06"></a>
## fmt

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 10], "end": [14, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f13edf99323a2982c9f12ead"></a>
## head

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::head` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L228).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:228`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42366f31bd91de9d5237df5"></a>
## list_from

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::list_from` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:191`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05e07463eb55c87c37728487"></a>
## new

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 1], "end": [35, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c44d1e4e454211e64edf7ab"></a>
## put

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::put` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:215`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b177eb853c4924074b564f6c"></a>
## read_files

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::read_files` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:206`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read data specified by the start and end offset from the file.

This will return the data in the same order as the provided file slices.

Multiple reads may occur in parallel, depending on the configured readahead.
See [`Self::with_readahead`](../operations/buoyant_kernel_engine.filesystem.ObjectStoreStorageHandler.md#op-f109c6acb314b9bdd5ceeae4).

<a id="op-f109c6acb314b9bdd5ceeae4"></a>
## with_readahead

`function` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::with_readahead` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_readahead(self, readahead: usize) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler", "path": "ObjectStoreStorageHandler"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 1], "end": [35, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum number of files to read in parallel.

<a id="op-f0a99cab15b8c9c51b81dd93"></a>
## inner

`struct_field` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::inner` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<delta_kernel::object_store::DynObjectStore>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4814d639ea991231be4fa05"></a>
## readahead

`struct_field` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::readahead` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
readahead: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9795437b994fb00e99749549"></a>
## task_executor

`struct_field` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler::task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_executor: std::sync::Arc<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/filesystem.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/filesystem.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
