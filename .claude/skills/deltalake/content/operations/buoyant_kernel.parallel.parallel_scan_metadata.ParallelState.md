# `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_scan_metadata.ParallelState.json).

<a id="op-5ee61419aa87dda59d758e45"></a>
## ParallelState

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParallelState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L119).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:119`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

State for parallel scan metadata processing.

This state can be serialized and distributed to remote workers, or wrapped
in Arc and shared across threads for local parallel processing.

<a id="op-1a652e48e26f77845e2b86f4"></a>
## file_read_schema

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::file_read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_read_schema(&self) -> SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L179).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:179`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the schema to use for reading checkpoint files.

Returns the checkpoint read schema which may have stats excluded
if skip_stats was enabled when the scan was created.

<a id="op-c1258c16ef9ad9f6c6204c24"></a>
## from_bytes

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::from_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_bytes(engine: &dyn Engine, bytes: &[u8]) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reconstruct a ParallelState from bytes.

This is a convenience method that combines JSON deserialization with
`from_serializable_state()`. The bytes must have been produced by `into_bytes()`.

# Parameters
- `engine`: Engine for creating evaluators and filters
- `bytes`: The serialized bytes from a previous `into_bytes()` call

<a id="op-ce231175bba0a6f27c548dc5"></a>
## from_serializable_state

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::from_serializable_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_serializable_state(engine: &dyn Engine, state: SerializableScanState) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:203`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reconstruct a ParallelState from serialized state.

# Parameters
- `engine`: Engine for creating evaluators and filters
- `state`: The serialized state from a previous `into_serializable_state()` call

<a id="op-cbe90815de4334df5621b1ca"></a>
## into_bytes

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::into_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_bytes(self) -> DeltaResult<Vec<u8>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L227).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serialize the processor state directly to bytes.

This is a convenience method that combines `into_serializable_state()` with
JSON serialization. For more control over serialization format, use
`into_serializable_state()` directly.

# Errors
Returns an error if the state cannot be serialized.

<a id="op-23bab98d9f3afdd035218534"></a>
## into_serializable_state

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::into_serializable_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_serializable_state(self) -> DeltaResult<SerializableScanState>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L192).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:192`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serialize the processor state for distributed processing.

Returns a `SerializableScanState` containing all information needed to
reconstruct this state on remote compute nodes.

# Errors
Returns an error if the state cannot be serialized (e.g., contains opaque predicates).

<a id="op-27c59cfb26a0c77e1ccfea9a"></a>
## log_metrics

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::log_metrics` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_metrics(&self)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L161).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelState", "path": "ParallelState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [247, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Log the accumulated metrics from parallel processing.

Call this after all parallel workers complete. The metrics will be logged
in the current tracing span context.

# Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::scan::ParallelState;
# use tracing::instrument;
#[instrument(skip_all, name = "parallel_scan")]
async fn process(state: Arc<ParallelState>) {
    // ... spawn workers that share Arc<ParallelState> ...
    // ... wait for workers to complete ...

    // Log accumulated metrics
    state.log_metrics();
}
```

<a id="op-3bd081836cb62f22a6f8a87c"></a>
## correlation_id

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L126).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:126`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque, caller-supplied correlation id inherited from the sequential phase. Does not
survive the serialization boundary; a reconstructed state carries `None` (tracked in
#2736).

<a id="op-916605024434934958c71bf9"></a>
## inner

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: scan::log_replay::ScanLogReplayProcessor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L120).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35b3ab36c1136b4c76485e6e"></a>
## operation_id

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_id: metrics::MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Operation ID inherited from the sequential phase for event correlation.

<a id="op-7ab232bd952cd0de82e0ac69"></a>
## parallel_start

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState::parallel_start` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parallel_start: std::time::Instant
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L128).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Start time for the parallel phase, set when this state is created.
