# `parquet::column::page_store::InMemoryPageStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.InMemoryPageStoreFactory.json).

<a id="op-d9200fd88c6a992e0a79494d"></a>
## InMemoryPageStoreFactory

`struct` · `parquet::column::page_store::InMemoryPageStoreFactory` · parquet 59.3.0

```rust
struct InMemoryPageStoreFactory
```

Source: `src/column/page_store.rs:194`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Factory for [`InMemoryPageStore`](../operations/parquet.column.page_store.InMemoryPageStore.md#op-591fcf71081a9ebdf4c5471b) — the default used by
[`ArrowWriter`](crate::arrow::arrow_writer::ArrowWriter).

<a id="op-79fb53ee9b1365f76bceb700"></a>
## create

`function` · `parquet::column::page_store::InMemoryPageStoreFactory::create` · parquet 59.3.0

```rust
fn create(&self, _args: &PageStoreArgs<'_>) -> Result<Box<dyn PageStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStoreFactory", "path": "InMemoryPageStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [200, 2], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "parquet::column::page_store::PageStoreFactory", "path": "PageStoreFactory"}, "trait_path": "parquet::column::page_store::PageStoreFactory"}`

Source: `src/column/page_store.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-482e6766d8f63de0ffa3aeb4"></a>
## default

`function` · `parquet::column::page_store::InMemoryPageStoreFactory::default` · parquet 59.3.0

```rust
fn default() -> InMemoryPageStoreFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStoreFactory", "path": "InMemoryPageStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 17], "end": [193, 24], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/column/page_store.rs:193`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca85f6e7726d7eb3173b9e1"></a>
## fmt

`function` · `parquet::column::page_store::InMemoryPageStoreFactory::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::InMemoryPageStoreFactory", "path": "InMemoryPageStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 10], "end": [193, 15], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column/page_store.rs:193`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
