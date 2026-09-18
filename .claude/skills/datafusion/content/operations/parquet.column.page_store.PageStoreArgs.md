# `parquet::column::page_store::PageStoreArgs`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.PageStoreArgs.json).

<a id="op-47aa3371bcf69c7ae7c0e721"></a>
## PageStoreArgs

`struct` · `parquet::column::page_store::PageStoreArgs` · parquet 59.3.0

```rust
struct PageStoreArgs<'a>
```

Source: `src/column/page_store.rs:111`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Context for a single [`PageStoreFactory::create`](../operations/parquet.column.page_store.PageStoreFactory.md#op-b449d1bbb45210fdf5b93f55) call.

Describes the leaf column chunk the store will buffer. It is held by
reference for the duration of the call; a backend reads only what it needs.
More fields may be added in future releases without breaking existing
implementations — the type is constructed only by the writer, so an
implementer only ever receives one and calls its accessors.

<a id="op-52c07c410012afa20dcd48b0"></a>
## column_descriptor

`function` · `parquet::column::page_store::PageStoreArgs::column_descriptor` · parquet 59.3.0

```rust
fn column_descriptor(&self) -> &ColumnDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::column::page_store::PageStoreArgs", "path": "PageStoreArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [143, 2], "filename": "src/column/page_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page_store.rs:140`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Descriptor for the leaf column: physical/logical type, path, and max
definition/repetition levels.

Lets a backend tailor buffering to the column — for example spilling only
large `BYTE_ARRAY` columns while keeping small fixed-width ones on the
heap.

<a id="op-4263261afed1fa01fc9ef67d"></a>
## column_index

`function` · `parquet::column::page_store::PageStoreArgs::column_index` · parquet 59.3.0

```rust
fn column_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::column::page_store::PageStoreArgs", "path": "PageStoreArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [143, 2], "filename": "src/column/page_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page_store.rs:130`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Index of the leaf column within the row group.

A backend may use this to e.g. name spill files or shard across a bounded
pool; it carries no ordering or coordination requirement.
