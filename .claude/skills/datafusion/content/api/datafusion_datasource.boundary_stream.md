# `datafusion_datasource::boundary_stream`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.boundary_stream.json`](../model/datafusion_datasource.boundary_stream.json)

## END_SCAN_LOOKAHEAD

`constant` · `datafusion_datasource::boundary_stream::END_SCAN_LOOKAHEAD`

```rust
const END_SCAN_LOOKAHEAD: u64 = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.boundary_stream.END_SCAN_LOOKAHEAD.md).


How far past `raw_end` the initial bounded fetch covers. If the terminating
newline is not found within this window, `ScanningLastTerminator` issues
successive same-sized GETs until the newline is located or EOF is reached.

---

## AlignedBoundaryStream

`struct` · `datafusion_datasource::boundary_stream::AlignedBoundaryStream`

```rust
struct AlignedBoundaryStream
```

**Implements**: `futures_core::stream::Stream`

**Methods** (1)

```rust
async fn new(store: Arc<dyn ObjectStore>, location: object_store::path::Path, raw_start: u64, raw_end: u64, file_size: u64, terminator: u8) -> object_store::Result<Self>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.boundary_stream.AlignedBoundaryStream.md).


A stream wrapper that lazily aligns byte boundaries to newline characters.

Given a raw byte stream starting from `fetch_start` (which is `start - 1`
for non-zero starts, or `0`), this stream:

1. Skips bytes until the first newline is found (start alignment)
2. Passes through data until the `end` boundary is reached
3. Continues past `end` to find the terminating newline (end alignment)

When the initial byte stream is exhausted during step 3 and the file has
not been fully read, `ScanningLastTerminator` issues additional bounded
`get_opts` calls (`END_SCAN_LOOKAHEAD` bytes each) until the newline is
found or EOF is reached.

---
