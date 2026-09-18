# `parquet::column::page_store::InMemoryPageStore`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.InMemoryPageStore.json).

<a id="op-591fcf71081a9ebdf4c5471b"></a>
## InMemoryPageStore

`struct` · `parquet::column::page_store::InMemoryPageStore` · parquet 59.3.0

```rust
struct InMemoryPageStore
```

Source: `src/column/page_store.rs:159`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The default [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631), holding blobs on the heap in a `Vec<Bytes>`.

Peak memory grows with the row group size; use a spilling backend to bound
it.

<a id="op-5dd2dcf18ca074b937e57992"></a>
## default

`function` · `parquet::column::page_store::InMemoryPageStore::default` · parquet 59.3.0

```rust
fn default() -> InMemoryPageStore
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStore", "path": "InMemoryPageStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 17], "end": [158, 24], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/column/page_store.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d63def6fd681e0227a1bbd0b"></a>
## fmt

`function` · `parquet::column::page_store::InMemoryPageStore::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStore", "path": "InMemoryPageStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 10], "end": [158, 15], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column/page_store.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c3f21ebfe38dc39fdc1f24d"></a>
## memory_size

`function` · `parquet::column::page_store::InMemoryPageStore::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStore", "path": "InMemoryPageStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [189, 2], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "parquet::column::page_store::PageStore", "path": "PageStore"}, "trait_path": "parquet::column::page_store::PageStore"}`

Source: `src/column/page_store.rs:186`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb185bbeada3e127119da28c"></a>
## put

`function` · `parquet::column::page_store::InMemoryPageStore::put` · parquet 59.3.0

```rust
fn put(&mut self, value: Bytes) -> Result<PageKey>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStore", "path": "InMemoryPageStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [189, 2], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "parquet::column::page_store::PageStore", "path": "PageStore"}, "trait_path": "parquet::column::page_store::PageStore"}`

Source: `src/column/page_store.rs:166`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a232699558fe9390a592196"></a>
## take

`function` · `parquet::column::page_store::InMemoryPageStore::take` · parquet 59.3.0

```rust
fn take(&mut self, key: PageKey) -> Result<Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStore", "path": "InMemoryPageStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [189, 2], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "parquet::column::page_store::PageStore", "path": "PageStore"}, "trait_path": "parquet::column::page_store::PageStore"}`

Source: `src/column/page_store.rs:173`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
