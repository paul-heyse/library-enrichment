# `object_store::buffered::BufWriter`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.buffered.BufWriter.json).

<a id="op-f092e35da4d4adde7c2b58f9"></a>
## BufWriter

`struct` · `object_store::buffered::BufWriter` · object_store 0.13.2

```rust
struct BufWriter
```

Source: `src/buffered.rs:221`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An async buffered writer compatible with the tokio IO traits

This writer adaptively uses [`ObjectStore::put_opts`](../operations/object_store.ObjectStore.md#op-4938238191db15fe350c71b9) or
[`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336) depending on the amount of data that has
been written.

Up to `capacity` bytes will be buffered in memory, and flushed on shutdown
using [`ObjectStore::put_opts`](../operations/object_store.ObjectStore.md#op-4938238191db15fe350c71b9). If `capacity` is exceeded, data will instead be
streamed using [`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336).

<a id="op-68afbbb7a9364e55f3add0da"></a>
## abort

`function` · `object_store::buffered::BufWriter::abort` · object_store 0.13.2

```rust
async fn abort(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:366`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Abort this writer, cleaning up any partially uploaded state

# Panic

Panics if this writer has already been shutdown or aborted

<a id="op-be9eeb7115f716fd8be81682"></a>
## fmt

`function` · `object_store::buffered::BufWriter::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [237, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffered.rs:232`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8320fb3db1b1101d31d89fd6"></a>
## new

`function` · `object_store::buffered::BufWriter::new` · object_store 0.13.2

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:252`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`BufWriter`](../operations/object_store.buffered.BufWriter.md#op-f092e35da4d4adde7c2b58f9) from the provided [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) and [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

<a id="op-7fed8d3f01e76466e1687276"></a>
## poll_flush

`function` · `object_store::buffered::BufWriter::poll_flush` · object_store 0.13.2

```rust
fn poll_flush(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [476, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_write::AsyncWrite", "path": "AsyncWrite"}, "trait_path": "tokio::io::async_write::AsyncWrite"}`

Source: `src/buffered.rs:424`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-849e084d6b1bdaf789340d6d"></a>
## poll_shutdown

`function` · `object_store::buffered::BufWriter::poll_shutdown` · object_store 0.13.2

```rust
fn poll_shutdown(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [476, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_write::AsyncWrite", "path": "AsyncWrite"}, "trait_path": "tokio::io::async_write::AsyncWrite"}`

Source: `src/buffered.rs:437`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769959e6543030a79492d85e"></a>
## poll_write

`function` · `object_store::buffered::BufWriter::poll_write` · object_store 0.13.2

```rust
fn poll_write(Pin<&mut self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [476, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_write::AsyncWrite", "path": "AsyncWrite"}, "trait_path": "tokio::io::async_write::AsyncWrite"}`

Source: `src/buffered.rs:376`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41953db3a68a6b9adc1d0e19"></a>
## put

`function` · `object_store::buffered::BufWriter::put` · object_store 0.13.2

```rust
async fn put(&mut self, bytes: Bytes) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:313`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Write data to the writer in [`Bytes`].

Unlike [`AsyncWrite::poll_write`], `put` can write data without extra copying.

This API is recommended while the data source generates [`Bytes`].

Unresolved upstream links (retained, not inferred): ``Bytes``, ``AsyncWrite::poll_write``.

<a id="op-ba9273a3a3ffc3fee63fe6fc"></a>
## with_attributes

`function` · `object_store::buffered::BufWriter::with_attributes` · object_store 0.13.2

```rust
fn with_attributes(self, attributes: Attributes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:280`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the attributes of the uploaded object

<a id="op-9d24912fee155926350e6960"></a>
## with_capacity

`function` · `object_store::buffered::BufWriter::with_capacity` · object_store 0.13.2

```rust
fn with_capacity(store: Arc<dyn ObjectStore>, path: Path, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:257`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`BufWriter`](../operations/object_store.buffered.BufWriter.md#op-f092e35da4d4adde7c2b58f9) from the provided [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca), [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) and `capacity`

<a id="op-3b82be5d50211c0b3ef5d7db"></a>
## with_extensions

`function` · `object_store::buffered::BufWriter::with_extensions` · object_store 0.13.2

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:301`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the extensions of the uploaded object

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

<a id="op-8388c115f0fe4d1f29832d12"></a>
## with_max_concurrency

`function` · `object_store::buffered::BufWriter::with_max_concurrency` · object_store 0.13.2

```rust
fn with_max_concurrency(self, max_concurrency: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:272`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Override the maximum number of in-flight requests for this writer

Defaults to 8

<a id="op-287d388167087ea7f10e98cd"></a>
## with_tags

`function` · `object_store::buffered::BufWriter::with_tags` · object_store 0.13.2

```rust
fn with_tags(self, tags: TagSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [373, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:288`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the tags of the uploaded object
