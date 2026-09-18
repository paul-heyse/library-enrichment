# `arrow_ord::sort::SortColumn`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.SortColumn.json).

<a id="op-9692442ca2a4243246c73618"></a>
## SortColumn

`struct` · `arrow_ord::sort::SortColumn` · arrow-ord 59.3.0

```rust
struct SortColumn
```

Source: `src/sort.rs:871`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

One column to be used in lexicographical sort

<a id="op-935524d4c2d85a0928632686"></a>
## clone

`function` · `arrow_ord::sort::SortColumn::clone` · arrow-ord 59.3.0

```rust
fn clone(&self) -> SortColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::sort::SortColumn", "path": "SortColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 10], "end": [870, 15], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort.rs:870`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e936b1f4f13d6c2dd0687b4"></a>
## fmt

`function` · `arrow_ord::sort::SortColumn::fmt` · arrow-ord 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::sort::SortColumn", "path": "SortColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 17], "end": [870, 22], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort.rs:870`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bd672132eae9447db8f8225"></a>
## options

`struct_field` · `arrow_ord::sort::SortColumn::options` · arrow-ord 59.3.0

```rust
options: Option<SortOptions>
```

Source: `src/sort.rs:875`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort options for this column

<a id="op-0a5b209dbe019909a335d131"></a>
## values

`struct_field` · `arrow_ord::sort::SortColumn::values` · arrow-ord 59.3.0

```rust
values: ArrayRef
```

Source: `src/sort.rs:873`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

The column to sort
