# `buoyant_kernel::parallel::sequential_phase::SequentialPhase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.sequential_phase.SequentialPhase.json).

<a id="op-11628dd71802282e8f6f34a6"></a>
## SequentialPhase

`struct` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SequentialPhase<P: LogReplayProcessor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Sequential log replay processor for parallel execution.

This iterator processes log replay sequentially:
1. Commit files (JSON)
2. Manifest (single-part checkpoint, if present)

After exhaustion, call `finish()` to extract:
- The processor (for serialization and distribution)
- Files (sidecars or multi-part checkpoint parts) for parallel processing

# Type Parameters
- `P`: A [`LogReplayProcessor`](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md#op-e6ee4714f432a749429a820c) implementation that processes action batches

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

<a id="op-49a6eff14d0886dc70418ce9"></a>
## Item

`assoc_type` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<<P as LogReplayProcessor>::Output, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L184).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::sequential_phase::SequentialPhase", "path": "SequentialPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [203, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:184`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2b1f2839f554c5aea9c3596"></a>
## finish

`function` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::finish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finish(self) -> DeltaResult<AfterSequential<P>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L149).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::sequential_phase::SequentialPhase", "path": "SequentialPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:149`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Complete sequential phase and extract processor + files for distribution.

Must be called after the iterator is exhausted.

# Returns
- `Done`: All processing done sequentially - no parallel phase needed
- `Parallel`: Parallel phase needed. The resulting files may be processed in parallel.

# Errors
Returns an error if called before iterator exhaustion.

<a id="op-c42ec8edff903a45958c67d5"></a>
## next

`function` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L186).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::sequential_phase::SequentialPhase", "path": "SequentialPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [203, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:186`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cf8a0c0955d5d9d6ffdc51b"></a>
## try_new

`function` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(processor: P, log_segment: &LogSegment, engine: Arc<dyn Engine>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "buoyant_kernel::parallel::sequential_phase::SequentialPhase", "path": "SequentialPhase"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new sequential phase log replay.

# Parameters
- `processor`: The log replay processor
- `log_segment`: The log segment to process
- `engine`: Engine for reading files

<a id="op-3692d4f2cac87b4e7a5b94ed"></a>
## checkpoint_manifest_phase

`struct_field` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::checkpoint_manifest_phase` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_manifest_phase: Option<log_reader::checkpoint_manifest::CheckpointManifestReader>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9861717dc6fd190d8a2c9dcf"></a>
## checkpoint_parts

`struct_field` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::checkpoint_parts` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_parts: Vec<FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0338b9349563ed068ca4f9d9"></a>
## commit_phase

`struct_field` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::commit_phase` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_phase: Option<log_reader::commit::CommitReader>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L72).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:72`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23dd9c1acfc6874df21cc2c"></a>
## is_finished

`struct_field` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::is_finished` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
is_finished: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L77).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70ffa341ab60149709f8a762"></a>
## processor

`struct_field` · `buoyant_kernel::parallel::sequential_phase::SequentialPhase::processor` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
processor: P
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
