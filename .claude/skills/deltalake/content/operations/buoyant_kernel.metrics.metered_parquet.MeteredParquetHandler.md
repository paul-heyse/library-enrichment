# `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_parquet.MeteredParquetHandler.json).

<a id="op-0ff1413b5221a5a8440c7dda"></a>
## MeteredParquetHandler

`struct` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MeteredParquetHandler
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Decorator over an engine-provided `Arc<dyn ParquetHandler>` that emits a
`ParquetReadCompleted` span on every `read_parquet_files` call.
`read_parquet_footer` and `write_parquet_file` are pass-through and emit nothing.

<a id="op-8a32ed4ac1eb00862ed2e88c"></a>
## fmt

`function` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler", "path": "MeteredParquetHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-858e166be52daf1d7bd32578"></a>
## new

`function` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(inner: Arc<dyn ParquetHandler>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler", "path": "MeteredParquetHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [34, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap `inner`. Debug-asserts that `inner` is not already a [`MeteredParquetHandler`](../operations/buoyant_kernel.metrics.metered_parquet.MeteredParquetHandler.md#op-0ff1413b5221a5a8440c7dda)
so spans are emitted exactly once.

<a id="op-861e037463a95945b8093ea2"></a>
## read_parquet_files

`function` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::read_parquet_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler", "path": "MeteredParquetHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [74, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2d606920886cad855c8aa88"></a>
## read_parquet_footer

`function` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::read_parquet_footer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler", "path": "MeteredParquetHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [74, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-083c83289c7c07854cf76ff7"></a>
## write_parquet_file

`function` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::write_parquet_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler", "path": "MeteredParquetHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [74, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ParquetHandler", "path": "ParquetHandler"}, "trait_path": "buoyant_kernel::ParquetHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9086c281569cf711aefb4cd6"></a>
## inner

`struct_field` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
