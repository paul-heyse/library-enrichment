# `parquet::basic::EncodingMask`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.EncodingMask.json).

<a id="op-588eb4c7204a05455dc8135b"></a>
## EncodingMask

`struct` · `parquet::basic::EncodingMask` · parquet 59.3.0

```rust
struct EncodingMask
```

Source: `src/basic.rs:511`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A bitmask representing the [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40)s employed while encoding a Parquet column chunk.

The Parquet [`ColumnMetaData`] struct contains an array that indicates what encodings were
used when writing that column chunk. For memory and performance reasons, this crate reduces
that array to bitmask, where each bit position represents a different [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40). This
struct contains that bitmask, and provides methods to interact with the data.

# Example
```no_run
# use parquet::file::metadata::ParquetMetaDataReader;
# use parquet::basic::Encoding;
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
// read parquet metadata from a file
let file = open_parquet_file("some_path.parquet");
let mut reader = ParquetMetaDataReader::new();
reader.try_parse(&file).unwrap();
let metadata = reader.finish().unwrap();

// find the encodings used by the first column chunk in the first row group
let col_meta = metadata.row_group(0).column(0);
let encodings = col_meta.encodings_mask();

// check to see if a particular encoding was used
let used_rle = encodings.is_set(Encoding::RLE);

// check to see if all of a set of encodings were used
let used_all = encodings.all_set([Encoding::RLE, Encoding::PLAIN].iter());

// convert mask to a Vec<Encoding>
let encodings_vec = encodings.encodings().collect::<Vec<_>>();
```

[`ColumnMetaData`]: https://github.com/apache/parquet-format/blob/9fd57b59e0ce1a82a69237dcf8977d3e72a2965d/src/main/thrift/parquet.thrift#L875

<a id="op-0ec6e8e4c2a850591cefe97b"></a>
## all_set

`function` · `parquet::basic::EncodingMask::all_set` · parquet 59.3.0

```rust
fn all_set<'a>(&self, encodings: impl Iterator<Item = &'a Encoding>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:561`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Test if all [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40)s in a given set are present in this mask.

<a id="op-275e93148ae6d7ce26d689dc"></a>
## as_i32

`function` · `parquet::basic::EncodingMask::as_i32` · parquet 59.3.0

```rust
fn as_i32(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:532`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return an integer representation of this `EncodingMask`.

<a id="op-b37ae71177b644e526eedb6b"></a>
## clone

`function` · `parquet::basic::EncodingMask::clone` · parquet 59.3.0

```rust
fn clone(&self) -> EncodingMask
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 17], "end": [510, 22], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:510`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87e91bd7d71f817b3ca81b51"></a>
## default

`function` · `parquet::basic::EncodingMask::default` · parquet 59.3.0

```rust
fn default() -> EncodingMask
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 30], "end": [510, 37], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/basic.rs:510`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d40a9bc546723bd7a72bcb"></a>
## encodings

`function` · `parquet::basic::EncodingMask::encodings` · parquet 59.3.0

```rust
fn encodings(&self) -> impl Iterator<Item = Encoding>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:566`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return an iterator over all [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40)s present in this mask.

<a id="op-4387e06e2eec5a705566cb52"></a>
## eq

`function` · `parquet::basic::EncodingMask::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &EncodingMask) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 39], "end": [510, 48], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:510`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd70546c3e0063ebcc1359be"></a>
## fmt

`function` · `parquet::basic::EncodingMask::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 10], "end": [510, 15], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:510`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13a026300eb8268771d29ee0"></a>
## insert

`function` · `parquet::basic::EncodingMask::insert` · parquet 59.3.0

```rust
fn insert(&mut self, val: Encoding)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:546`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Mark the given [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40) as present in this mask.

<a id="op-3b90e85454de784912fdae2c"></a>
## is_only

`function` · `parquet::basic::EncodingMask::is_only` · parquet 59.3.0

```rust
fn is_only(&self, val: Encoding) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:556`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Test if this mask has only the bit for the given [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40) set.

<a id="op-6a1afacd0d9b890afdcaf8fa"></a>
## is_set

`function` · `parquet::basic::EncodingMask::is_set` · parquet 59.3.0

```rust
fn is_set(&self, val: Encoding) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:551`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Test if a given [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40) is present in this mask.

<a id="op-233331b3ba0109f5b7518d78"></a>
## new_from_encodings

`function` · `parquet::basic::EncodingMask::new_from_encodings` · parquet 59.3.0

```rust
fn new_from_encodings<'a>(encodings: impl Iterator<Item = &'a Encoding>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:537`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `EncodingMask` from a collection of [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40)s.

<a id="op-3e862633aaea865fbc84528f"></a>
## try_new

`function` · `parquet::basic::EncodingMask::try_new` · parquet 59.3.0

```rust
fn try_new(val: i32) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EncodingMask", "path": "EncodingMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [513, 1], "end": [575, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:524`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Attempt to create a new `EncodingMask` from an integer.

This will return an error if a bit outside the allowable range is set.
