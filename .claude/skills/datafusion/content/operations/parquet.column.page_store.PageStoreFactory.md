# `parquet::column::page_store::PageStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.PageStoreFactory.json).

<a id="op-e91a99482c4168f6c3f8f594"></a>
## PageStoreFactory

`trait` · `parquet::column::page_store::PageStoreFactory` · parquet 59.3.0

```rust
trait PageStoreFactory: Send + Sync + Debug
```

Source: `src/column/page_store.rs:149`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a fresh [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631) for each column chunk.

See
[`ArrowWriterOptions::with_page_store_factory`](crate::arrow::arrow_writer::ArrowWriterOptions::with_page_store_factory).

<a id="op-b449d1bbb45210fdf5b93f55"></a>
## create

`function` · `parquet::column::page_store::PageStoreFactory::create` · parquet 59.3.0

```rust
fn create(&self, args: &PageStoreArgs<'_>) -> Result<Box<dyn PageStore>>
```

Source: `src/column/page_store.rs:151`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new, empty [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631) for the leaf column described by `args`.
