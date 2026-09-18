# `parquet::file::page_index::column_index::PrimitiveColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.column_index.PrimitiveColumnIndex.json).

<a id="op-52513b04c52143e896d1e7fc"></a>
## PrimitiveColumnIndex

`struct` · `parquet::file::page_index::column_index::PrimitiveColumnIndex` · parquet 59.3.0

```rust
struct PrimitiveColumnIndex<T>
```

Source: `src/file/page_index/column_index.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column index for primitive types

<a id="op-d8ea1e4601fa3d805ae46e37"></a>
## Target

`assoc_type` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::Target` · parquet 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [256, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/file/page_index/column_index.rs:251`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daf2a5374eedf06b63aa20f0"></a>
## clone

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PrimitiveColumnIndex<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 22], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/column_index.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26f83c3410adead7c79c33c0"></a>
## deref

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::deref` · parquet 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [256, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/file/page_index/column_index.rs:253`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecd829e2e911b226b5ca4d02"></a>
## eq

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &PrimitiveColumnIndex<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 24], "end": [90, 33], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/column_index.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62817c3bc6b6d3d5ffb7391f"></a>
## fmt

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/column_index.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3ca298dddde82ce0dfe07ec"></a>
## max_value

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::max_value` · parquet 59.3.0

```rust
fn max_value(&self, idx: usize) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:241`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the max value for the page indexed by `idx`

It is `None` when all values are null

<a id="op-610467b3306f1eb2832e8c32"></a>
## max_values

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::max_values` · parquet 59.3.0

```rust
fn max_values(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:195`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an array containing the max values for each page.

Values in the returned slice are only valid if [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52)
is `false` for the same index.

<a id="op-3535fc3533712c811449b449"></a>
## max_values_iter

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(&self) -> impl Iterator<Item = Option<&T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:215`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an iterator over the max values.

Values may be `None` when [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52) is `true`.

<a id="op-62ce3b32cef6a0475d21894b"></a>
## min_value

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::min_value` · parquet 59.3.0

```rust
fn min_value(&self, idx: usize) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the min value for the page indexed by `idx`

It is `None` when all values are null

<a id="op-f71cdf226de2d85276410861"></a>
## min_values

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::min_values` · parquet 59.3.0

```rust
fn min_values(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:187`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an array containing the min values for each page.

Values in the returned slice are only valid if [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52)
is `false` for the same index.

<a id="op-c98248ef99f3edcc870029a6"></a>
## min_values_iter

`function` · `parquet::file::page_index::column_index::PrimitiveColumnIndex::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(&self) -> impl Iterator<Item = Option<&T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::page_index::column_index::PrimitiveColumnIndex", "path": "PrimitiveColumnIndex"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [248, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:202`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an iterator over the min values.

Values may be `None` when [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52) is `true`.
