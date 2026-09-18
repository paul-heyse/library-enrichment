# `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_engine.MeteredDeltaEngine.json).

<a id="op-6a6a81013b8e6f35873abf9f"></a>
## MeteredDeltaEngine

`struct` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MeteredDeltaEngine
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Decorator over any [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) that meters its storage, JSON, and Parquet handlers.
See module docs.

<a id="op-7e7b2b7c0c734ef986e82341"></a>
## evaluation_handler

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::evaluation_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3434050be1db86ce0204244f"></a>
## fmt

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [67, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e25496bb09af69e088573db9"></a>
## json_handler

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::json_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_handler(&self) -> Arc<dyn JsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L78).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5facc97d989c309639e1ad9e"></a>
## new

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(inner: Arc<dyn Engine>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap `inner`. Debug-asserts that none of `inner`'s handlers are already metered
wrappers, so the resulting engine emits each span exactly once.

The check is shallow: it inspects the immediate concrete type returned by each
handler accessor and does not walk intermediate wrapper types. Wrapping a metered
handler behind a non-metered wrapper before re-wrapping (e.g.
`Metered(Foo(Metered(...)))`) silently double-counts.

<a id="op-74fba74a215ec80f974445b2"></a>
## parquet_handler

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::parquet_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L82).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:82`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e23c9b4baf5c874c05b1e7c"></a>
## storage_handler

`function` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::storage_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine", "path": "MeteredDeltaEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-508962657f662cbea9aebd6a"></a>
## inner

`struct_field` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<dyn Engine>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8cb6d8b5aee33d00ff3623e"></a>
## json

`struct_field` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::json` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json: std::sync::Arc<dyn JsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2eae5c421f0e48f2cfda1bc9"></a>
## parquet

`struct_field` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::parquet` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parquet: std::sync::Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L21).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed612ea6e8cb02edd3e74042"></a>
## storage

`struct_field` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine::storage` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage: std::sync::Arc<dyn StorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
