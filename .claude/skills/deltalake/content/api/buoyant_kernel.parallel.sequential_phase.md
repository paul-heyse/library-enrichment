# `buoyant_kernel::parallel::sequential_phase`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.parallel.sequential_phase.json`](../model/buoyant_kernel.parallel.sequential_phase.json)

## AfterSequential

`enum` · `buoyant_kernel::parallel::sequential_phase::AfterSequential`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.parallel.sequential_phase.AfterSequential.md)

Also reachable as `delta_kernel::parallel::sequential_phase::AfterSequential`

```rust
enum AfterSequential<P: LogReplayProcessor>
```

**Variants**: `Done`, `Parallel`

Result of sequential log replay processing.
cbindgen:ignore

---

## SequentialPhase

`struct` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.parallel.sequential_phase.SequentialPhase.md)

Also reachable as `delta_kernel::parallel::sequential_phase::SequentialPhase`

```rust
struct SequentialPhase<P: LogReplayProcessor>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (2)

```rust
fn finish(self) -> DeltaResult<AfterSequential<P>>
fn try_new(processor: P, log_segment: &LogSegment, engine: Arc<dyn Engine>) -> DeltaResult<Self>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Sequential log replay processor for parallel execution.

This iterator processes log replay sequentially:
1. Commit files (JSON)
2. Manifest (single-part checkpoint, if present)

After exhaustion, call `finish()` to extract:
- The processor (for serialization and distribution)
- Files (sidecars or multi-part checkpoint parts) for parallel processing

# Type Parameters
- `P`: A [`LogReplayProcessor`] implementation that processes action batches

# Example

```ignore
let mut sequential = SequentialPhase::try_new(processor, log_segment, engine)?;

// Iterate over sequential batches
for batch in sequential.by_ref() {
    let metadata = batch?;
    // Process metadata
}

// Extract processor and files for distribution (if needed)
match sequential.finish()? {
    AfterSequential::Parallel { processor, files } => {
        // Parallel phase needed - distribute files for parallel processing.
        // If crossing the network boundary, the processor must be serialized.
        let serialized = processor.serialize()?;
        let partitions = partition_files(files, num_workers);
        for (worker, partition) in partitions {
            worker.send(serialized.clone(), partition)?;
        }
    }
    AfterSequential::Done(processor) => {
        // No parallel phase needed - all processing complete sequentially
        println!("Log replay complete");
    }
}
```
cbindgen:ignore

---
