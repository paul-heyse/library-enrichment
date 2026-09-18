# `datafusion_datasource::boundary_stream::AlignedBoundaryStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.boundary_stream.AlignedBoundaryStream.json).

<a id="op-07f52935072c17fd47be5e48"></a>
## AlignedBoundaryStream

`struct` · `datafusion_datasource::boundary_stream::AlignedBoundaryStream` · datafusion-datasource 55.1.0

```rust
struct AlignedBoundaryStream
```

Source: `src/boundary_stream.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

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

<a id="op-85bc3c2b4837fedd97172486"></a>
## Item

`assoc_type` · `datafusion_datasource::boundary_stream::AlignedBoundaryStream::Item` · datafusion-datasource 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::boundary_stream::AlignedBoundaryStream", "path": "AlignedBoundaryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [396, 2], "filename": "src/boundary_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/boundary_stream.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09d22cbc7bb519c6abe125e2"></a>
## new

`function` · `datafusion_datasource::boundary_stream::AlignedBoundaryStream::new` · datafusion-datasource 55.1.0

```rust
async fn new(store: Arc<dyn ObjectStore>, location: object_store::path::Path, raw_start: u64, raw_end: u64, file_size: u64, terminator: u8) -> object_store::Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::boundary_stream::AlignedBoundaryStream", "path": "AlignedBoundaryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [214, 2], "filename": "src/boundary_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/boundary_stream.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Open a ranged byte stream from `store` and return a ready-to-poll
`AlignedBoundaryStream`.

Issues a single bounded `get_opts` call covering
`[fetch_start, raw_end + END_SCAN_LOOKAHEAD)`.  If the terminating
newline is not found within that window, `ScanningLastTerminator`
automatically issues additional `END_SCAN_LOOKAHEAD`-sized GETs
via `store` until the newline is found or EOF is reached.

<a id="op-673a82a6d812ea881b24b10a"></a>
## poll_next

`function` · `datafusion_datasource::boundary_stream::AlignedBoundaryStream::poll_next` · datafusion-datasource 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::boundary_stream::AlignedBoundaryStream", "path": "AlignedBoundaryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [396, 2], "filename": "src/boundary_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/boundary_stream.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
