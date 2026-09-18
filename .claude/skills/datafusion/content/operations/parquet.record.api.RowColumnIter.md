# `parquet::record::api::RowColumnIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.RowColumnIter.json).

<a id="op-03a0cad15769a7d35620ce4e"></a>
## RowColumnIter

`struct` · `parquet::record::api::RowColumnIter` · parquet 59.3.0

```rust
struct RowColumnIter<'a>
```

Source: `src/record/api.rs:122`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

`RowColumnIter` represents an iterator over column names and values in a Row.

<a id="op-6e66d8b8cb4dffcd3ee3876b"></a>
## Item

`assoc_type` · `parquet::record::api::RowColumnIter::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::api::RowColumnIter", "path": "RowColumnIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [139, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/api.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e447f5953bf3d6bcd9d26eb"></a>
## next

`function` · `parquet::record::api::RowColumnIter::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::api::RowColumnIter", "path": "RowColumnIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [139, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/api.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
