# `arrow_row::RowsIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.RowsIter.json).

<a id="op-8ef2da04ac8f83458156275b"></a>
## RowsIter

`struct` · `arrow_row::RowsIter` · arrow-row 59.3.0

```rust
struct RowsIter<'a>
```

Source: `src/lib.rs:1491`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

An iterator over [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

<a id="op-c7228ffc77b7833650975fcc"></a>
## Item

`assoc_type` · `arrow_row::RowsIter::Item` · arrow-row 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1497, 1], "end": [1515, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/lib.rs:1498`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a4168c1dfd3d6baf2586306"></a>
## fmt

`function` · `arrow_row::RowsIter::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1490, 10], "end": [1490, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1490`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3bbe11324f2b0f159d32b45"></a>
## len

`function` · `arrow_row::RowsIter::len` · arrow-row 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1517, 1], "end": [1521, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::iter::traits::exact_size::ExactSizeIterator", "path": "ExactSizeIterator"}, "trait_path": "core::iter::traits::exact_size::ExactSizeIterator"}`

Source: `src/lib.rs:1518`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e0f0445a1d41838ea19144"></a>
## next

`function` · `arrow_row::RowsIter::next` · arrow-row 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1497, 1], "end": [1515, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/lib.rs:1500`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70540787f30b2dd625305070"></a>
## next_back

`function` · `arrow_row::RowsIter::next_back` · arrow-row 59.3.0

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1523, 1], "end": [1536, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/lib.rs:1524`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee94e79afa46d00fa0d03518"></a>
## size_hint

`function` · `arrow_row::RowsIter::size_hint` · arrow-row 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::RowsIter", "path": "RowsIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1497, 1], "end": [1515, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/lib.rs:1511`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
