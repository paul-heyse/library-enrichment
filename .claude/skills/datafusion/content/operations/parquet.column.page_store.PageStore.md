# `parquet::column::page_store::PageStore`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.PageStore.json).

<a id="op-049980600cf52224c7af9631"></a>
## PageStore

`trait` · `parquet::column::page_store::PageStore` · parquet 59.3.0

```rust
trait PageStore: Send
```

Source: `src/column/page_store.rs:79`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A pluggable store for completed, serialized page blobs.

The store is intentionally "dumb": it only maps an opaque [`PageKey`](../operations/parquet.column.page_store.PageKey.md#op-ed2b8f23a50733178579bb70) to a
blob of bytes. It knows nothing about pages, dictionaries, ordering, or
offsets. The caller keeps the handles it gets back from [`put`](Self::put)
and decides what they mean.

Each store instance is owned by a single column writer and mutated by one
thread at a time (both methods take `&mut self`), so it needs no internal
synchronization — hence only `Send`, not `Sync`.

The default ([`InMemoryPageStore`](../operations/parquet.column.page_store.InMemoryPageStore.md#op-591fcf71081a9ebdf4c5471b)) keeps blobs in memory on the heap.

For an example of configuring the Parquet writer to use an alternate
`PageStore` see the [`ArrowWriterOptions::with_page_store_factory`] API.

[`ArrowWriterOptions::with_page_store_factory`]: crate::arrow::arrow_writer::ArrowWriterOptions::with_page_store_factory

<a id="op-3107ae96a47e73adf5e49316"></a>
## memory_size

`function` · `parquet::column::page_store::PageStore::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Source: `src/column/page_store.rs:99`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of bytes this store currently holds **in memory** (resident
on the heap), used to report the writer's memory footprint.

The default is `0`, which is exactly right for a backend that moves
every blob off-heap (a temp file, object storage): the bytes it has been
handed no longer occupy heap. The in-memory backend overrides this to
report its resident blobs. A backend that keeps a partial in-memory
buffer should report that buffer's size.

<a id="op-fd813a80d838c2a0b30772d3"></a>
## put

`function` · `parquet::column::page_store::PageStore::put` · parquet 59.3.0

```rust
fn put(&mut self, value: Bytes) -> Result<PageKey>
```

Source: `src/column/page_store.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Store `value`, returning a handle that can later be passed to
[`take`](Self::take).

<a id="op-79a96af825cfccfb78a7bdf1"></a>
## take

`function` · `parquet::column::page_store::PageStore::take` · parquet 59.3.0

```rust
fn take(&mut self, key: PageKey) -> Result<Bytes>
```

Source: `src/column/page_store.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Take back the blob previously stored under `key`.

The caller takes ownership of the returned bytes and will **not** request
`key` again, so the store may release any resources backing it — eagerly
here, or when the store is dropped.
