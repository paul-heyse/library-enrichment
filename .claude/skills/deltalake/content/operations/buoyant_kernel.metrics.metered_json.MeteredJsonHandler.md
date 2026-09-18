# `buoyant_kernel::metrics::metered_json::MeteredJsonHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_json.MeteredJsonHandler.json).

<a id="op-4917f5cfca39dfff79dc6c93"></a>
## MeteredJsonHandler

`struct` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MeteredJsonHandler
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Decorator over an engine-provided `Arc<dyn JsonHandler>` that emits a
`JsonReadCompleted` span on every `read_json_files` call. `parse_json` and
`write_json_file` are pass-through and emit nothing.

<a id="op-3b67887ef3960887747dd523"></a>
## fmt

`function` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_json::MeteredJsonHandler", "path": "MeteredJsonHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a300b40c8ac0d3ed68ecc8"></a>
## new

`function` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(inner: Arc<dyn JsonHandler>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_json::MeteredJsonHandler", "path": "MeteredJsonHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [34, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap `inner`. Debug-asserts that `inner` is not already a [`MeteredJsonHandler`](../operations/buoyant_kernel.metrics.metered_json.MeteredJsonHandler.md#op-4917f5cfca39dfff79dc6c93)
so spans are emitted exactly once.

<a id="op-67f8c5ab37ae6bc12bf0e590"></a>
## parse_json

`function` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L43).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_json::MeteredJsonHandler", "path": "MeteredJsonHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78faa479d8b90c7ab60170ac"></a>
## read_json_files

`function` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::read_json_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_json::MeteredJsonHandler", "path": "MeteredJsonHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fe140531b20aa8a3c7530cc"></a>
## write_json_file

`function` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::write_json_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_json_file(&self, path: &url::Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_json::MeteredJsonHandler", "path": "MeteredJsonHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs"}, "trait": {"args": null, "id": "buoyant_kernel::JsonHandler", "path": "JsonHandler"}, "trait_path": "buoyant_kernel::JsonHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce13c84b38f1e736790ccf8"></a>
## inner

`struct_field` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<dyn JsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
