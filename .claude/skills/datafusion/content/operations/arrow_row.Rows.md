# `arrow_row::Rows`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.Rows.json).

<a id="op-21ce3501b5312cc762135114"></a>
## Rows

`struct` · `arrow_row::Rows` · arrow-row 59.3.0

```rust
struct Rows
```

Source: `src/lib.rs:1331`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

A row-oriented representation of arrow data, that is normalized for comparison.

See the [module level documentation](self) and [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) for more details.

<a id="op-fbcdd48b9eb99c9721503e6e"></a>
## clear

`function` · `arrow_row::Rows::clear` · arrow-row 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1401`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Sets the length of this [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) to 0

<a id="op-d4fe66b7e04b2fa5c6066d9c"></a>
## clone

`function` · `arrow_row::Rows::clone` · arrow-row 59.3.0

```rust
fn clone(&self) -> Rows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1330, 17], "end": [1330, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1330`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb3b22f8a0bc09c933c73aa5"></a>
## fmt

`function` · `arrow_row::Rows::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1330, 10], "end": [1330, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1330`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd3ceb9f1fdf926ea641773b"></a>
## iter

`function` · `arrow_row::Rows::iter` · arrow-row 59.3.0

```rust
fn iter(&self) -> RowsIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1412`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns an iterator over the [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) in this [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

<a id="op-95641c05b7cecabe89a314ec"></a>
## lengths

`function` · `arrow_row::Rows::lengths` · arrow-row 59.3.0

```rust
fn lengths(&self) -> RowLengthIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1396`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Get an iterator over the lengths of each row in this [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

<a id="op-4927ffcedc0dce7b88b4e88c"></a>
## num_rows

`function` · `arrow_row::Rows::num_rows` · arrow-row 59.3.0

```rust
fn num_rows(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1407`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the number of [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) in this [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

<a id="op-27aa82c65f6899f9e62a6e62"></a>
## push

`function` · `arrow_row::Rows::push` · arrow-row 59.3.0

```rust
fn push(&mut self, row: Row<'_>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1345`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Append a [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) to this [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

<a id="op-e2d0899a3217caafc6aad70a"></a>
## reserve

`function` · `arrow_row::Rows::reserve` · arrow-row 59.3.0

```rust
fn reserve(&mut self, row_capacity: usize, data_capacity: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1356`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Reserve capacity for `row_capacity` rows with a total length of `data_capacity`

<a id="op-a68ff1b0248146a54ce48377"></a>
## row

`function` · `arrow_row::Rows::row` · arrow-row 59.3.0

```rust
fn row(&self, row: usize) -> Row<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1362`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the row at index `row`

<a id="op-e4d5db44d044d3898c5512bb"></a>
## row_len

`function` · `arrow_row::Rows::row_len` · arrow-row 59.3.0

```rust
fn row_len(&self, row: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1389`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the number of bytes the row at index `row` is occupying,
that is, what is the length of the returned [`Row::data`](../operations/arrow_row.Row.md#op-81b83917fead6c7afc103f47) will be.

<a id="op-5d2219500262ebcfef8d9874"></a>
## row_unchecked

`function` · `arrow_row::Rows::row_unchecked` · arrow-row 59.3.0

```rust
unsafe fn row_unchecked(&self, index: usize) -> Row<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1377`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the row at `index` without bounds checking

# Safety
Caller must ensure that `index + 1` is less than the number of offsets (#rows + 1)

<a id="op-80a87e01a306109ad49b931f"></a>
## size

`function` · `arrow_row::Rows::size` · arrow-row 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1419`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the size of this instance in bytes

Includes the size of `Self`.

<a id="op-d57f90ed5c877b8671c535c3"></a>
## try_into_binary

`function` · `arrow_row::Rows::try_into_binary` · arrow-row 59.3.0

```rust
fn try_into_binary(self) -> Result<BinaryArray, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::Rows", "path": "Rows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1474, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1455`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create a [BinaryArray](../operations/arrow_array.array.binary_array.BinaryArray.md#op-7129d9dc68ce402f881c540c) from the [Rows](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) data without reallocating the
underlying bytes.


```
# use std::sync::Arc;
# use std::collections::HashSet;
# use arrow_array::cast::AsArray;
# use arrow_array::StringArray;
# use arrow_row::{OwnedRow, Row, RowConverter, RowParser, SortField};
# use arrow_schema::DataType;
#
let converter = RowConverter::new(vec![SortField::new(DataType::Utf8)]).unwrap();
let array = StringArray::from(vec!["hello", "world", "a", "a", "hello"]);
let rows = converter.convert_columns(&[Arc::new(array)]).unwrap();

// We can convert rows into binary format and back.
let values: Vec<OwnedRow> = rows.iter().map(|r| r.owned()).collect();
let binary = rows.try_into_binary().expect("known-small array");
let parser = converter.parser();
let parsed: Vec<OwnedRow> =
  binary.iter().flatten().map(|b| parser.parse(b).owned()).collect();
assert_eq!(values, parsed);
```

# Errors

This function will return an error if there is more data than can be stored in
a [BinaryArray](../operations/arrow_array.array.binary_array.BinaryArray.md#op-7129d9dc68ce402f881c540c) -- i.e. if the total data size is more than 2GiB.
