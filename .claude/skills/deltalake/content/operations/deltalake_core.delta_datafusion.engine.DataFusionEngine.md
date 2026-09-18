# `deltalake_core::delta_datafusion::engine::DataFusionEngine`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.engine.DataFusionEngine.json).

<a id="op-d2d1cbbd0c8d1b847114a037"></a>
## DataFusionEngine

`struct` · `deltalake_core::delta_datafusion::engine::DataFusionEngine` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DataFusionEngine
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L20).

Source: `crates/core/src/delta_datafusion/engine/mod.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Datafusion based Kernel Engine

<a id="op-80f327e6347475c6aa7cc35c"></a>
## clone

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DataFusionEngine
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 10], "end": [19, 15], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6abc2ed961f2eadb636800bb"></a>
## evaluation_handler

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::evaluation_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [67, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01612ef32d2282ad6329eed2"></a>
## json_handler

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::json_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_handler(&self) -> Arc<dyn JsonHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [67, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-357c5bf2be9a9f12acf93520"></a>
## new

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(ctx: Arc<TaskContext>, handle: Handle) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [49, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create an engine from an explicit [`TaskContext`] and Tokio runtime [`Handle`].

The other constructors delegate here; call this directly when you need to bind the
engine to a specific runtime handle rather than the ambient one.

Unresolved upstream links (retained, not inferred): ``TaskContext``, ``Handle``.

<a id="op-ab6b9ed03866e15ae8eb9cdb"></a>
## new_from_context

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::new_from_context` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_from_context(ctx: Arc<TaskContext>) -> Arc<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [49, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create an engine directly from a DataFusion [`TaskContext`], using the current Tokio
runtime handle. Useful inside physical operators where only the task context is
available.

Unresolved upstream links (retained, not inferred): ``TaskContext``.

<a id="op-3f3eccb48bc47927e089fed8"></a>
## new_from_session

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::new_from_session` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_from_session(session: &dyn Session) -> Arc<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [49, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create an engine from a DataFusion [`Session`], reusing its task context and the
current Tokio runtime handle. This is the convenient entry point when wiring the
kernel engine into an active query session.

Unresolved upstream links (retained, not inferred): ``Session``.

<a id="op-05d4fd6ef0234b02fb1a6946"></a>
## parquet_handler

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::parquet_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [67, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54998be8e9d4d97e425208da"></a>
## storage_handler

`function` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::storage_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L56).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::engine::DataFusionEngine", "path": "DataFusionEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [67, 2], "filename": "crates/core/src/delta_datafusion/engine/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `crates/core/src/delta_datafusion/engine/mod.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5232c0c7fc636f62f35fa85c"></a>
## formats

`struct_field` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::formats` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
formats: std::sync::Arc<self::file_formats::DataFusionFileFormatHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L22).

Source: `crates/core/src/delta_datafusion/engine/mod.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c316c59f21f1fc570c0abc55"></a>
## storage

`struct_field` · `deltalake_core::delta_datafusion::engine::DataFusionEngine::storage` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage: std::sync::Arc<self::storage::DataFusionStorageHandler>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L21).

Source: `crates/core/src/delta_datafusion/engine/mod.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
