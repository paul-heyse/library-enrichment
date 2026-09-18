# `parquet::record::reader::ReaderIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.reader.ReaderIter.json).

<a id="op-3573ad9df25ec282a86f281b"></a>
## ReaderIter

`struct` · `parquet::record::reader::ReaderIter` · parquet 59.3.0

```rust
struct ReaderIter
```

Source: `src/record/reader.rs:822`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Internal iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)s for a reader.

<a id="op-e2eada5578e0d51e0ed87e2a"></a>
## Item

`assoc_type` · `parquet::record::reader::ReaderIter::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::ReaderIter", "path": "ReaderIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [838, 1], "end": [849, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/reader.rs:839`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2e7af1e5f1603e4507546a"></a>
## next

`function` · `parquet::record::reader::ReaderIter::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Result<Row>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::ReaderIter", "path": "ReaderIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [838, 1], "end": [849, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/reader.rs:841`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
