# `object_store::buffered`

Crate `object_store` · 3 public items · structured records in [`model/object_store.buffered.json`](../model/object_store.buffered.json)

## DEFAULT_BUFFER_SIZE

`constant` · `object_store::buffered::DEFAULT_BUFFER_SIZE`

```rust
const DEFAULT_BUFFER_SIZE: usize = _
```

[Full member, field, variant and typed contracts](../operations/object_store.buffered.DEFAULT_BUFFER_SIZE.md).


The default buffer size used by [`BufReader`]

---

## BufReader

`struct` · `object_store::buffered::BufReader`

```rust
struct BufReader
```

**Implements**: `tokio::io::async_buf_read::AsyncBufRead`, `tokio::io::async_read::AsyncRead`, `tokio::io::async_seek::AsyncSeek`

**Derives**: Debug

**Methods** (2)

```rust
fn new(store: Arc<dyn ObjectStore>, meta: &ObjectMeta) -> Self
fn with_capacity(store: Arc<dyn ObjectStore>, meta: &ObjectMeta, capacity: usize) -> Self
```

**via `tokio::io::async_buf_read::AsyncBufRead`**

```rust
fn consume(Pin<&mut self>, amt: usize)
fn poll_fill_buf(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>>
```

**via `tokio::io::async_read::AsyncRead`**

```rust
fn poll_read(Pin<&mut self>, cx: &mut Context<'_>, out: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>>
```

**via `tokio::io::async_seek::AsyncSeek`**

```rust
fn poll_complete(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<u64>>
fn start_seek(Pin<&mut self>, position: SeekFrom) -> std::io::Result<()>
```

[Full member, field, variant and typed contracts](../operations/object_store.buffered.BufReader.md).


An async-buffered reader compatible with the tokio IO traits

Internally this maintains a buffer of the requested size, and uses [`ObjectStoreExt::get_range`]
to populate its internal buffer once depleted. This buffer is cleared on seek.

Whilst simple, this interface will typically be outperformed by the native [`ObjectStore`]
methods that better map to the network APIs. This is because most object stores have
very [high first-byte latencies], on the order of 100-200ms, and so avoiding unnecessary
round-trips is critical to throughput.

Systems looking to sequentially scan a file should instead consider using [`ObjectStoreExt::get`],
or [`ObjectStore::get_opts`], or [`ObjectStoreExt::get_range`] to read a particular range.

Systems looking to read multiple ranges of a file should instead consider using
[`ObjectStore::get_ranges`], which will optimise the vectored IO.

[high first-byte latencies]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/optimizing-performance.html
[`ObjectStoreExt::get`]: crate::ObjectStoreExt::get

---

## BufWriter

`struct` · `object_store::buffered::BufWriter`

```rust
struct BufWriter
```

**Implements**: `tokio::io::async_write::AsyncWrite`

**Derives**: Debug

**Methods** (8)

```rust
async fn abort(&mut self) -> Result<()>
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
async fn put(&mut self, bytes: Bytes) -> Result<()>
fn with_attributes(self, attributes: Attributes) -> Self
fn with_capacity(store: Arc<dyn ObjectStore>, path: Path, capacity: usize) -> Self
fn with_extensions(self, extensions: Extensions) -> Self
fn with_max_concurrency(self, max_concurrency: usize) -> Self
fn with_tags(self, tags: TagSet) -> Self
```

**via `tokio::io::async_write::AsyncWrite`**

```rust
fn poll_flush(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>
fn poll_shutdown(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>
fn poll_write(Pin<&mut self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>>
```

[Full member, field, variant and typed contracts](../operations/object_store.buffered.BufWriter.md).


An async buffered writer compatible with the tokio IO traits

This writer adaptively uses [`ObjectStore::put_opts`] or
[`ObjectStore::put_multipart_opts`] depending on the amount of data that has
been written.

Up to `capacity` bytes will be buffered in memory, and flushed on shutdown
using [`ObjectStore::put_opts`]. If `capacity` is exceeded, data will instead be
streamed using [`ObjectStore::put_multipart_opts`].

---
