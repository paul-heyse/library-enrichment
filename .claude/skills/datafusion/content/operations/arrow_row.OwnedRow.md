# `arrow_row::OwnedRow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.OwnedRow.json).

<a id="op-ceb9c865a8e778c2667c5bf4"></a>
## OwnedRow

`struct` · `arrow_row::OwnedRow` · arrow-row 59.3.0

```rust
struct OwnedRow
```

Source: `src/lib.rs:1610`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Owned version of a [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) that can be moved/cloned freely.

This contains the data for the one specific row (not the entire buffer of all rows).

<a id="op-944449027f6acdf80a5224ae"></a>
## as_ref

`function` · `arrow_row::OwnedRow::as_ref` · arrow-row 59.3.0

```rust
fn as_ref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1664, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"primitive": "u8"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/lib.rs:1661`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71cc6bcd1ab0f095ab9ee3e1"></a>
## clone

`function` · `arrow_row::OwnedRow::clone` · arrow-row 59.3.0

```rust
fn clone(&self) -> OwnedRow
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1609, 17], "end": [1609, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1609`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc26cd6535df3fcb53b0f852"></a>
## cmp

`function` · `arrow_row::OwnedRow::cmp` · arrow-row 59.3.0

```rust
fn cmp(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1645, 1], "end": [1650, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/lib.rs:1647`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee9cba80e80b4098046960af"></a>
## eq

`function` · `arrow_row::OwnedRow::eq` · arrow-row 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1629, 1], "end": [1634, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1631`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4e698d8fc549a7aba431e3c"></a>
## fmt

`function` · `arrow_row::OwnedRow::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1609, 10], "end": [1609, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1609`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b2fa04f6e58ce28896a1c2c"></a>
## hash

`function` · `arrow_row::OwnedRow::hash` · arrow-row 59.3.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1652, 1], "end": [1657, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lib.rs:1654`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20a0275aa587adebb6f48ec7"></a>
## partial_cmp

`function` · `arrow_row::OwnedRow::partial_cmp` · arrow-row 59.3.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1638, 1], "end": [1643, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/lib.rs:1640`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cd27b70616d1218da3dbec9"></a>
## row

`function` · `arrow_row::OwnedRow::row` · arrow-row 59.3.0

```rust
fn row(&self) -> Row<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::OwnedRow", "path": "OwnedRow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1615, 1], "end": [1625, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1619`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Get borrowed [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) from owned version.

This is helpful if you want to compare an [`OwnedRow`](../operations/arrow_row.OwnedRow.md#op-ceb9c865a8e778c2667c5bf4) with a [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966).
