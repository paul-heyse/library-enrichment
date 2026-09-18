# `arrow_ord::sort::LexicographicalComparator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.LexicographicalComparator.json).

<a id="op-07161450869987f188b7237b"></a>
## LexicographicalComparator

`struct` · `arrow_ord::sort::LexicographicalComparator` · arrow-ord 59.3.0

```rust
struct LexicographicalComparator
```

Source: `src/sort.rs:1126`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data
at given two indices. The lifetime is the same at the data wrapped.

<a id="op-6e6ae4a7e8d814be82f24067"></a>
## compare

`function` · `arrow_ord::sort::LexicographicalComparator::compare` · arrow-ord 59.3.0

```rust
fn compare(&self, a_idx: usize, b_idx: usize) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::sort::LexicographicalComparator", "path": "LexicographicalComparator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1130, 1], "end": [1157, 2], "filename": "src/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort.rs:1132`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

lexicographically compare values at the wrapped columns with given indices.

<a id="op-a9817152fe37bf0b6a4cc76f"></a>
## try_new

`function` · `arrow_ord::sort::LexicographicalComparator::try_new` · arrow-ord 59.3.0

```rust
fn try_new(columns: &[SortColumn]) -> Result<LexicographicalComparator, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::sort::LexicographicalComparator", "path": "LexicographicalComparator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1130, 1], "end": [1157, 2], "filename": "src/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort.rs:1144`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Create a new lex comparator that will wrap the given sort columns and give comparison
results with two indices.
