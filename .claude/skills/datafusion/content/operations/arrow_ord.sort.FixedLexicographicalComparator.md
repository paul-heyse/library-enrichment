# `arrow_ord::sort::FixedLexicographicalComparator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.FixedLexicographicalComparator.json).

<a id="op-556ccfc7b4d3c8080ce053fc"></a>
## FixedLexicographicalComparator

`struct` · `arrow_ord::sort::FixedLexicographicalComparator` · arrow-ord 59.3.0

```rust
struct FixedLexicographicalComparator<const N: usize>
```

Source: `src/sort.rs:1162`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data
at given two indices. This version of the comparator is for compile-time constant number of columns.
The lifetime is the same at the data wrapped.

<a id="op-2193cdc7b2628881c84f0c9c"></a>
## compare

`function` · `arrow_ord::sort::FixedLexicographicalComparator::compare` · arrow-ord 59.3.0

```rust
fn compare(&self, a_idx: usize, b_idx: usize) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "N", "is_literal": false, "value": null}}], "constraints": []}}, "id": "arrow_ord::sort::FixedLexicographicalComparator", "path": "FixedLexicographicalComparator"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1166, 1], "end": [1201, 2], "filename": "src/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort.rs:1168`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

lexicographically compare values at the wrapped columns with given indices.

<a id="op-119e827eb6d14eb4099ebb2f"></a>
## try_new

`function` · `arrow_ord::sort::FixedLexicographicalComparator::try_new` · arrow-ord 59.3.0

```rust
fn try_new(columns: &[SortColumn]) -> Result<FixedLexicographicalComparator<N>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "N", "is_literal": false, "value": null}}], "constraints": []}}, "id": "arrow_ord::sort::FixedLexicographicalComparator", "path": "FixedLexicographicalComparator"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1166, 1], "end": [1201, 2], "filename": "src/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort.rs:1181`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Create a new lex comparator that will wrap the given sort columns and give comparison
results with two indices.
The number of columns should be equal to the compile-time constant N.
