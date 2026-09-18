# `arrow_select::filter::SlicesIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.SlicesIterator.json).

<a id="op-3e3c1df9c53f02d6520ff069"></a>
## SlicesIterator

`struct` · `arrow_select::filter::SlicesIterator` · arrow-select 59.3.0

```rust
struct SlicesIterator<'a>
```

Source: `src/filter.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

An iterator of `(usize, usize)` each representing an interval
`[start, end)` whose slots of a bitmap [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) are true.

Each interval corresponds to a contiguous region of memory to be
"taken" from an array to be filtered.

## Notes:

1. Ignores the validity bitmap (ignores nulls)

2. Only performant for filters that copy across long contiguous runs

<a id="op-324748a7a6100b9da7222098"></a>
## Item

`assoc_type` · `arrow_select::filter::SlicesIterator::Item` · arrow-select 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_select::filter::SlicesIterator", "path": "SlicesIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [78, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/filter.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ed289a340c8e5f65861759"></a>
## fmt

`function` · `arrow_select::filter::SlicesIterator::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_select::filter::SlicesIterator", "path": "SlicesIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5e4d580f3ba90010b16e28d"></a>
## from

`function` · `arrow_select::filter::SlicesIterator::from` · arrow-select 59.3.0

```rust
fn from(filter: &'a BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_select::filter::SlicesIterator", "path": "SlicesIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [70, 2], "filename": "src/filter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/filter.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b34e4aa85068c6263d694147"></a>
## new

`function` · `arrow_select::filter::SlicesIterator::new` · arrow-select 59.3.0

```rust
fn new(filter: &'a BooleanArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_select::filter::SlicesIterator", "path": "SlicesIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [64, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Creates a new iterator from a [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505)

<a id="op-05c1292c9034b05d3071910d"></a>
## next

`function` · `arrow_select::filter::SlicesIterator::next` · arrow-select 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_select::filter::SlicesIterator", "path": "SlicesIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [78, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/filter.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
