# `buoyant_kernel::parallel::parallel_phase::ParallelPhase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_phase.ParallelPhase.json).

<a id="op-e608a707f19a6b956dfee17e"></a>
## ParallelPhase

`struct` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParallelPhase<P: ParallelLogReplayProcessor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Processes checkpoint leaf files in parallel using a shared processor.

This struct is designed for distributed execution where checkpoint leaf files (sidecars or
multi-part checkpoint parts) are partitioned across multiple executors. Each executor creates
its own `ParallelPhase` instance with a subset of files, but all instances share the same
processor (typically wrapped in `Arc`) to coordinate deduplication.

Implements `Iterator` to yield processed batches. The processor is responsible for filtering
out actions for files already seen in the sequential_phase.

# Example workflow
- Partition leaf files across N executors
- Create one `ParallelPhase<Arc<Processor>>` per executor with its file subset
- Each instance processes its files independently while sharing deduplication state
cbindgen:ignore

<a id="op-c365ffc2707c964ef2b7d764"></a>
## Item

`assoc_type` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<<P as ParallelLogReplayProcessor>::Output, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L113).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::parallel_phase::ParallelPhase", "path": "ParallelPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [120, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb235acc42047a1ba43f7102"></a>
## file_read_schema

`function` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::file_read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_read_schema() -> SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::parallel_phase::ParallelPhase", "path": "ParallelPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [99, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:96`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the schema used for reading checkpoint files.

This schema defines the structure expected when reading checkpoint parquet files,
including the action types (add, remove, etc.) and their fields.

<a id="op-cd70a7b35c81de8543b71841"></a>
## new_from_iter

`function` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::new_from_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_from_iter(processor: P, iter: impl IntoIterator<Item = DeltaResult<Box<dyn EngineData>>> + 'static) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::parallel_phase::ParallelPhase", "path": "ParallelPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [99, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new parallel phase from an existing iterator of EngineData.

Use this constructor when you want to parallelize processing at the row group level.
Instead of reading entire checkpoint files, you can provide an iterator that yields
individual row groups, allowing finer-grained parallelization.

# Parameters
- `processor`: Shared processor (wrap in `Arc` for distribution across executors)
- `iter`: Iterator yielding checkpoint action batches, typically from individual row groups

<a id="op-b697bc117077d12a1ad90705"></a>
## next

`function` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L115).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::parallel_phase::ParallelPhase", "path": "ParallelPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [120, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c09a8f5a8fddec7187f12a5"></a>
## try_new

`function` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(engine: Arc<dyn Engine>, processor: P, leaf_files: Vec<FileMeta>, read_schema: SchemaRef) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L50).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::parallel_phase::ParallelPhase", "path": "ParallelPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [99, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:50`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new parallel phase for processing checkpoint leaf files.

# Parameters
- `engine`: Engine for reading parquet files
- `processor`: Shared processor (wrap in `Arc` for distribution across executors)
- `leaf_files`: Checkpoint leaf files (sidecars or multi-part checkpoint parts)
- `read_schema`: Schema to use for reading checkpoint files

<a id="op-a1c816748b1fe7bb758f7fae"></a>
## leaf_checkpoint_reader

`struct_field` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::leaf_checkpoint_reader` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
leaf_checkpoint_reader: Box<dyn Iterator<Item = DeltaResult<log_replay::ActionsBatch>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L37).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93f19b447d8c06ef78acb9e3"></a>
## processor

`struct_field` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase::processor` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
processor: P
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L36).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
