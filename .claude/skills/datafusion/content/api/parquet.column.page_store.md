# `parquet::column::page_store`

Crate `parquet` · 6 public items · structured records in [`model/parquet.column.page_store.json`](../model/parquet.column.page_store.json)

## InMemoryPageStore

`struct` · `parquet::column::page_store::InMemoryPageStore`

Also reachable as `parquet::arrow::arrow_writer::InMemoryPageStore`

```rust
struct InMemoryPageStore
```

**Implements**: `parquet::column::page_store::PageStore`

**Derives**: Debug, Default

**via `parquet::column::page_store::PageStore`**

```rust
fn memory_size(&self) -> usize
fn put(&mut self, value: Bytes) -> Result<PageKey>
fn take(&mut self, key: PageKey) -> Result<Bytes>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.InMemoryPageStore.md).


The default [`PageStore`], holding blobs on the heap in a `Vec<Bytes>`.

Peak memory grows with the row group size; use a spilling backend to bound
it.

---

## InMemoryPageStoreFactory

`struct` · `parquet::column::page_store::InMemoryPageStoreFactory`

Also reachable as `parquet::arrow::arrow_writer::InMemoryPageStoreFactory`

```rust
struct InMemoryPageStoreFactory
```

**Implements**: `parquet::column::page_store::PageStoreFactory`

**Derives**: Debug, Default

**via `parquet::column::page_store::PageStoreFactory`**

```rust
fn create(&self, _args: &PageStoreArgs<'_>) -> Result<Box<dyn PageStore>>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.InMemoryPageStoreFactory.md).


Factory for [`InMemoryPageStore`] — the default used by
[`ArrowWriter`](crate::arrow::arrow_writer::ArrowWriter).

---

## PageKey

`struct` · `parquet::column::page_store::PageKey`

Also reachable as `parquet::arrow::arrow_writer::PageKey`

```rust
struct PageKey
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
const fn get(self) -> u64
const fn new(raw: u64) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.PageKey.md).


An opaque, store-allocated handle to a blob held by a [`PageStore`].

Handles are allocated by the store — densely and sequentially — and are only
meaningful to the store that produced them. The caller treats them as opaque
tokens.

---

## PageStoreArgs

`struct` · `parquet::column::page_store::PageStoreArgs`

Also reachable as `parquet::arrow::arrow_writer::PageStoreArgs`

```rust
struct PageStoreArgs<'a>
```

**Methods** (2)

```rust
fn column_descriptor(&self) -> &ColumnDescriptor
fn column_index(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.PageStoreArgs.md).


Context for a single [`PageStoreFactory::create`] call.

Describes the leaf column chunk the store will buffer. It is held by
reference for the duration of the call; a backend reads only what it needs.
More fields may be added in future releases without breaking existing
implementations — the type is constructed only by the writer, so an
implementer only ever receives one and calls its accessors.

---

## PageStore

`trait` · `parquet::column::page_store::PageStore`

Also reachable as `parquet::arrow::arrow_writer::PageStore`

```rust
trait PageStore: Send
```

**Implementors** (1)

- `parquet::column::page_store::InMemoryPageStore`

**Methods** (3)

```rust
fn memory_size(&self) -> usize
fn put(&mut self, value: Bytes) -> Result<PageKey>
fn take(&mut self, key: PageKey) -> Result<Bytes>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.PageStore.md).


A pluggable store for completed, serialized page blobs.

The store is intentionally "dumb": it only maps an opaque [`PageKey`] to a
blob of bytes. It knows nothing about pages, dictionaries, ordering, or
offsets. The caller keeps the handles it gets back from [`put`](Self::put)
and decides what they mean.

Each store instance is owned by a single column writer and mutated by one
thread at a time (both methods take `&mut self`), so it needs no internal
synchronization — hence only `Send`, not `Sync`.

The default ([`InMemoryPageStore`]) keeps blobs in memory on the heap.

For an example of configuring the Parquet writer to use an alternate
`PageStore` see the [`ArrowWriterOptions::with_page_store_factory`] API.

[`ArrowWriterOptions::with_page_store_factory`]: crate::arrow::arrow_writer::ArrowWriterOptions::with_page_store_factory

---

## PageStoreFactory

`trait` · `parquet::column::page_store::PageStoreFactory`

Also reachable as `parquet::arrow::arrow_writer::PageStoreFactory`

```rust
trait PageStoreFactory: Send + Sync + Debug
```

**Implementors** (1)

- `parquet::column::page_store::InMemoryPageStoreFactory`

**Methods** (1)

```rust
fn create(&self, args: &PageStoreArgs<'_>) -> Result<Box<dyn PageStore>>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page_store.PageStoreFactory.md).


Creates a fresh [`PageStore`] for each column chunk.

See
[`ArrowWriterOptions::with_page_store_factory`](crate::arrow::arrow_writer::ArrowWriterOptions::with_page_store_factory).

---
