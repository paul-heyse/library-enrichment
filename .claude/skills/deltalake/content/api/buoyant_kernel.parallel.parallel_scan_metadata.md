# `buoyant_kernel::parallel::parallel_scan_metadata`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.parallel.parallel_scan_metadata.json`](../model/buoyant_kernel.parallel.parallel_scan_metadata.json)

## AfterSequentialScanMetadata

`enum` · `buoyant_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata`

Also reachable as `buoyant_kernel::scan::AfterSequentialScanMetadata`, `delta_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata`

```rust
enum AfterSequentialScanMetadata
```

**Variants**: `Done`, `Parallel`

Result of sequential scan metadata processing.

This enum indicates whether distributed processing is needed:
- `Done`: All processing completed sequentially - no distributed phase needed.
- `Parallel`: Contains state and files for parallel processing.

---

## ParallelScanMetadata

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata`

Also reachable as `buoyant_kernel::scan::ParallelScanMetadata`, `delta_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata`

```rust
struct ParallelScanMetadata
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (2)

```rust
fn new_from_iter(state: Arc<ParallelState>, iter: impl IntoIterator<Item = DeltaResult<Box<dyn EngineData>>> + 'static) -> Self
fn try_new(engine: Arc<dyn Engine>, state: Arc<ParallelState>, leaf_files: Vec<FileMeta>) -> DeltaResult<Self>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## ParallelState

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelState`

Also reachable as `buoyant_kernel::scan::ParallelState`, `delta_kernel::parallel::parallel_scan_metadata::ParallelState`

```rust
struct ParallelState
```

**Methods** (6)

```rust
fn file_read_schema(&self) -> SchemaRef
fn from_bytes(engine: &dyn Engine, bytes: &[u8]) -> DeltaResult<Self>
fn from_serializable_state(engine: &dyn Engine, state: SerializableScanState) -> DeltaResult<Self>
fn into_bytes(self) -> DeltaResult<Vec<u8>>
fn into_serializable_state(self) -> DeltaResult<SerializableScanState>
fn log_metrics(&self)
```

State for parallel scan metadata processing.

This state can be serialized and distributed to remote workers, or wrapped
in Arc and shared across threads for local parallel processing.

---

## SequentialScanMetadata

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata`

Also reachable as `buoyant_kernel::scan::SequentialScanMetadata`, `delta_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata`

```rust
struct SequentialScanMetadata
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn finish(self) -> DeltaResult<AfterSequentialScanMetadata>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Sequential scan metadata processing.

This phase processes commits and single-part checkpoint manifests sequentially.
After exhaustion, call `finish()` to get the result which indicates whether
a distributed phase is needed.

---
