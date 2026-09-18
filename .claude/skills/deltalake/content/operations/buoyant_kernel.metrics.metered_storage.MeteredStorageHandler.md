# `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_storage.MeteredStorageHandler.json).

<a id="op-649a125cd3d23861605596b7"></a>
## MeteredStorageHandler

`struct` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MeteredStorageHandler
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Decorator over an engine-provided `Arc<dyn StorageHandler>` that emits the kernel's
standard `"storage"` spans on operations that produce metrics. `put`, `head`, and `delete`
are pass-through and emit nothing.

<a id="op-1ad568a7c270940896661f4f"></a>
## copy_atomic

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::copy_atomic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f672496af18b55118a981b"></a>
## delete

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::delete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete(&self, path: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L86).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:86`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99713e06af74ef1ad82d1bf9"></a>
## fmt

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [42, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa20c8de0a01519bcdcc0c15"></a>
## head

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::head` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L82).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:82`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b7ad7c409ddfe7cc13ff0f"></a>
## list_from

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::list_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L45).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7c3563674fec61483bb60e2"></a>
## new

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(inner: Arc<dyn StorageHandler>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [35, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap `inner`. Debug-asserts that `inner` is not already a
[`MeteredStorageHandler`](../operations/buoyant_kernel.metrics.metered_storage.MeteredStorageHandler.md#op-649a125cd3d23861605596b7) so spans are emitted exactly once.

<a id="op-c5eaa45002384f14bbebc3cb"></a>
## put

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::put` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L78).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38676cba61b92ba8472960d4"></a>
## read_files

`function` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::read_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_storage::MeteredStorageHandler", "path": "MeteredStorageHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [89, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs"}, "trait": {"args": null, "id": "buoyant_kernel::StorageHandler", "path": "StorageHandler"}, "trait_path": "buoyant_kernel::StorageHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:58`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b5fcb56a97bf24ddb715ab3"></a>
## inner

`struct_field` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<dyn StorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L21).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
