# `arrow_row::Row`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.Row.json).

<a id="op-a32be7fffe0a916746ead966"></a>
## Row

`struct` · `arrow_row::Row` · arrow-row 59.3.0

```rust
struct Row<'a>
```

Source: `src/lib.rs:1547`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

A comparable representation of a row.

See the [module level documentation](self) for more details.

Two [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) can only be compared if they both belong to [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)
returned by calls to [`RowConverter::convert_columns`](../operations/arrow_row.RowConverter.md#op-610a34b7b5b57c4d480dfde7) on the same
[`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3). If different [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3)s are used, any
ordering established by comparing the [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) is arbitrary.

<a id="op-19f62fd1e29407804f2dd6b6"></a>
## as_ref

`function` · `arrow_row::Row::as_ref` · arrow-row 59.3.0

```rust
fn as_ref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1599, 1], "end": [1604, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"primitive": "u8"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/lib.rs:1601`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-735e858f09bbb14eeeed0c04"></a>
## clone

`function` · `arrow_row::Row::clone` · arrow-row 59.3.0

```rust
fn clone(&self) -> Row<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1546, 23], "end": [1546, 28], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1546`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a544d25d6c835894cf48e2a5"></a>
## cmp

`function` · `arrow_row::Row::cmp` · arrow-row 59.3.0

```rust
fn cmp(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [1590, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/lib.rs:1587`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b83917fead6c7afc103f47"></a>
## data

`function` · `arrow_row::Row::data` · arrow-row 59.3.0

```rust
fn data(&self) -> &'a [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1552, 1], "end": [1565, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1562`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

The row's bytes, with the lifetime of the underlying data.

<a id="op-9feb8a1459f665d4b6f10f4f"></a>
## eq

`function` · `arrow_row::Row::eq` · arrow-row 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1569, 1], "end": [1574, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1571`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5b89990cee055fec71b3319"></a>
## fmt

`function` · `arrow_row::Row::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1546, 10], "end": [1546, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1546`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cbed817ec18096f6690b66d"></a>
## hash

`function` · `arrow_row::Row::hash` · arrow-row 59.3.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1592, 1], "end": [1597, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lib.rs:1594`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bea3c7762ef6bf1c320a5d18"></a>
## owned

`function` · `arrow_row::Row::owned` · arrow-row 59.3.0

```rust
fn owned(&self) -> OwnedRow
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1552, 1], "end": [1565, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1554`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create owned version of the row to detach it from the shared [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114).

<a id="op-d35c9084ec0e0f54be8c177e"></a>
## partial_cmp

`function` · `arrow_row::Row::partial_cmp` · arrow-row 59.3.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_row::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1578, 1], "end": [1583, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/lib.rs:1580`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
