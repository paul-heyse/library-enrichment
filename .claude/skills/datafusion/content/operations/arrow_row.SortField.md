# `arrow_row::SortField`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.SortField.json).

<a id="op-e58d2d9b989df785f7f8db59"></a>
## SortField

`struct` · `arrow_row::SortField` · arrow-row 59.3.0

```rust
struct SortField
```

Source: `src/lib.rs:930`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Configure the data type and sort order for a given column

<a id="op-bc25c5434394432bc86b2f6c"></a>
## clone

`function` · `arrow_row::SortField::clone` · arrow-row 59.3.0

```rust
fn clone(&self) -> SortField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [929, 17], "end": [929, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:929`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a105e28c704136adcf115c1"></a>
## eq

`function` · `arrow_row::SortField::eq` · arrow-row 59.3.0

```rust
fn eq(&self, other: &SortField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [929, 24], "end": [929, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:929`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13f5452db1c7e164a8015610"></a>
## fmt

`function` · `arrow_row::SortField::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [929, 10], "end": [929, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:929`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bd493f5d95e6e0ee14264a7"></a>
## new

`function` · `arrow_row::SortField::new` · arrow-row 59.3.0

```rust
fn new(data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [954, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:939`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create a new column with the given data type

<a id="op-d60f6e26936aff2067bd7549"></a>
## new_with_options

`function` · `arrow_row::SortField::new_with_options` · arrow-row 59.3.0

```rust
fn new_with_options(data_type: DataType, options: SortOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [954, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:944`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create a new column with the given data type and [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949)

<a id="op-c228303fa43e4028237b4d07"></a>
## size

`function` · `arrow_row::SortField::size` · arrow-row 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::SortField", "path": "SortField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [954, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:951`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Return size of this instance in bytes.

Includes the size of `Self`.
