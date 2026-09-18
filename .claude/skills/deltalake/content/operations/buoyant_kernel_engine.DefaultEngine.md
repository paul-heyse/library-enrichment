# `buoyant_kernel_engine::DefaultEngine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.DefaultEngine.json).

<a id="op-6311b39f77a5eea5de61a8f5"></a>
## DefaultEngine

`struct` · `buoyant_kernel_engine::DefaultEngine` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultEngine<E: TaskExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L96).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:96`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc6e8bc384d8a25228a66bd9"></a>
## builder

`function` · `buoyant_kernel_engine::DefaultEngine::builder` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder(object_store: Arc<DynObjectStore>) -> DefaultEngineBuilder<DefaultTaskExecutor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "executor::tokio::TokioBackgroundExecutor"}}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [224, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a [`DefaultEngineBuilder`](../operations/buoyant_kernel_engine.DefaultEngineBuilder.md#op-d6de93f6e92c25ea359b026f) for constructing a [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5) with custom options.

# Parameters

- `object_store`: The object store to use.

<a id="op-4a5d21151b18cf37304dacbd"></a>
## default_parquet_handler

`function` · `buoyant_kernel_engine::DefaultEngine::default_parquet_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default_parquet_handler(&self) -> Arc<DefaultParquetHandler<E>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L279).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [313, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:279`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the concrete [`DefaultParquetHandler`](../operations/buoyant_kernel_engine.parquet.DefaultParquetHandler.md#op-e65eba96a1e47c1a6710e3b6) for callers that need the inherent
async `write_parquet_file` helper not exposed by the [`ParquetHandler`](../operations/buoyant_kernel.ParquetHandler.md#op-935e1b04f902c9a95af7c376) trait.
For the metered trait surface used by reads, use [`Self::parquet_handler`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-b6e742f2f06060e9bc902fb0).

TODO(#2701): lift the inherent helper onto [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5) so this accessor
(and the `raw_parquet` field) can be removed.

<a id="op-e85527f1836914826716a05a"></a>
## enter

`function` · `buoyant_kernel_engine::DefaultEngine::enter` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enter(&self) -> <E as TaskExecutor>::Guard<'_>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L265).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [313, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:265`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Enter the runtime context of the executor associated with this engine.

# Panics

When calling `enter` multiple times, the returned guards **must** be dropped in the reverse
order that they were acquired.  Failure to do so will result in a panic and possible memory
leaks.

<a id="op-4e9685e2cbf051a5fa61d018"></a>
## evaluation_handler

`function` · `buoyant_kernel_engine::DefaultEngine::evaluation_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L336).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [351, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:336`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4293e1617cb418a183fb476f"></a>
## fmt

`function` · `buoyant_kernel_engine::DefaultEngine::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L95).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:95`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9671a0bcb03a63018d59b1f"></a>
## get_object_store_for_url

`function` · `buoyant_kernel_engine::DefaultEngine::get_object_store_for_url` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_object_store_for_url(&self, _url: &Url) -> Option<Arc<DynObjectStore>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L269).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [313, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:269`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fdef1a0ad15b792a1edd35"></a>
## json_handler

`function` · `buoyant_kernel_engine::DefaultEngine::json_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_handler(&self) -> Arc<dyn JsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L344).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [351, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:344`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e742f2f06060e9bc902fb0"></a>
## parquet_handler

`function` · `buoyant_kernel_engine::DefaultEngine::parquet_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L348).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [351, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:348`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-118cfc27faff03525d8a01f5"></a>
## storage_handler

`function` · `buoyant_kernel_engine::DefaultEngine::storage_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L340).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [351, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": {"args": null, "id": "buoyant_kernel::Engine", "path": "Engine"}, "trait_path": "buoyant_kernel::Engine"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:340`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38f91e3122ade4e62b180410"></a>
## write_parquet

`function` · `buoyant_kernel_engine::DefaultEngine::write_parquet` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_parquet(&self, data: &ArrowEngineData, write_context: &WriteContext) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L295).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "buoyant_kernel_engine::DefaultEngine", "path": "DefaultEngine"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [313, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:295`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write `data` as a parquet file using the provided `write_context`.

`data` must not contain partition columns. If the table materializes partition columns (e.g.
`materializePartitionColumns` or `icebergCompatV3`), this function automatically inserts
them into the data.

The `write_context` must be created by [`Transaction::partitioned_write_context`] or
[`Transaction::unpartitioned_write_context`], which handle partition value validation,
serialization, and logical-to-physical key translation.

[`Transaction::partitioned_write_context`]: delta_kernel::transaction::Transaction::partitioned_write_context
[`Transaction::unpartitioned_write_context`]: delta_kernel::transaction::Transaction::unpartitioned_write_context

Unresolved upstream links (retained, not inferred): `delta_kernel::transaction::Transaction::partitioned_write_context`, `delta_kernel::transaction::Transaction::unpartitioned_write_context`.

<a id="op-8d458fc5fef12f21dee5fda2"></a>
## evaluation

`struct_field` · `buoyant_kernel_engine::DefaultEngine::evaluation` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
evaluation: std::sync::Arc<delta_kernel::engine::arrow_expression::ArrowEvaluationHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L106).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce5384262341bde0960e1226"></a>
## json

`struct_field` · `buoyant_kernel_engine::DefaultEngine::json` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json: std::sync::Arc<delta_kernel::metrics::MeteredJsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L100).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c05640401491ccacdc06512"></a>
## object_store

`struct_field` · `buoyant_kernel_engine::DefaultEngine::object_store` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_store: std::sync::Arc<delta_kernel::object_store::DynObjectStore>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L97).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-906b2ebc368d42429fc48e84"></a>
## parquet

`struct_field` · `buoyant_kernel_engine::DefaultEngine::parquet` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parquet: std::sync::Arc<delta_kernel::metrics::MeteredParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L101).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97e49ea5175a7f4da3a8e719"></a>
## raw_parquet

`struct_field` · `buoyant_kernel_engine::DefaultEngine::raw_parquet` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
raw_parquet: std::sync::Arc<self::parquet::DefaultParquetHandler<E>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L105).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:105`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Concrete parquet handler retained so [`Self::write_parquet`] and
[`Self::default_parquet_handler`] can reach the inherent `write_parquet_file`
helper, which the [`ParquetHandler`](../operations/buoyant_kernel.ParquetHandler.md#op-935e1b04f902c9a95af7c376) trait surface doesn't expose.

Unresolved upstream links (retained, not inferred): ``Self::write_parquet``, ``Self::default_parquet_handler``.

<a id="op-71078e89f17794636e8ec655"></a>
## storage

`struct_field` · `buoyant_kernel_engine::DefaultEngine::storage` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage: std::sync::Arc<delta_kernel::metrics::MeteredStorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L99).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c54bcec369eb215cb88e1739"></a>
## task_executor

`struct_field` · `buoyant_kernel_engine::DefaultEngine::task_executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_executor: std::sync::Arc<E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L98).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
