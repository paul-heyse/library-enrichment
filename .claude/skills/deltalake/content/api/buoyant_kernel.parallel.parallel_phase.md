# `buoyant_kernel::parallel::parallel_phase`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.parallel.parallel_phase.json`](../model/buoyant_kernel.parallel.parallel_phase.json)

## ParallelPhase

`struct` · `buoyant_kernel::parallel::parallel_phase::ParallelPhase`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.parallel.parallel_phase.ParallelPhase.md)

Also reachable as `delta_kernel::parallel::parallel_phase::ParallelPhase`

```rust
struct ParallelPhase<P: ParallelLogReplayProcessor>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (3)

```rust
fn file_read_schema() -> SchemaRef
fn new_from_iter(processor: P, iter: impl IntoIterator<Item = DeltaResult<Box<dyn EngineData>>> + 'static) -> Self
fn try_new(engine: Arc<dyn Engine>, processor: P, leaf_files: Vec<FileMeta>, read_schema: SchemaRef) -> DeltaResult<Self>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

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

---
