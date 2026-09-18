# `parquet::data_type::Int96`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.Int96.json).

<a id="op-c484d6686cf394a8abff5f20"></a>
## Int96

`struct` · `parquet::data_type::Int96` · parquet 59.3.0

```rust
struct Int96
```

Source: `src/data_type.rs:37`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Rust representation for logical type INT96, value is backed by an array of `u32`.
The type only takes 12 bytes, without extra padding.

<a id="op-f5de24262ad2760865f63601"></a>
## Item

`assoc_type` · `parquet::data_type::Int96::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "crate::data_type::Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [679, 50], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:679`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36751efcfb6aeb02de1c35c7"></a>
## as_bytes

`function` · `parquet::data_type::Int96::as_bytes` · parquet 59.3.0

```rust
fn as_bytes(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [634, 1], "end": [639, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}, "trait_path": "parquet::data_type::AsBytes"}`

Source: `src/data_type.rs:635`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ec4335b3c848b2df2c223e"></a>
## clone

`function` · `parquet::data_type::Int96::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Int96
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-982ce473de42d0d6ad417b01"></a>
## cmp

`function` · `parquet::data_type::Int96::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [158, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/data_type.rs:152`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Order `Int96` correctly for (deprecated) timestamp types.

Note: this is done even though the Int96 type is deprecated and the
[spec does not define the sort order]
because some engines, notably Spark and Databricks Photon still write
Int96 timestamps and rely on their order for optimization.

[spec does not define the sort order]: https://github.com/apache/parquet-format/blob/cf943c197f4fad826b14ba0c40eb0ffdab585285/src/main/thrift/parquet.thrift#L1079

<a id="op-aa9d8699e3a1c6cc596ddd11"></a>
## data

`function` · `parquet::data_type::Int96::data` · parquet 59.3.0

```rust
fn data(&self) -> &[u32]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:67`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns underlying data as slice of [`u32`].

Unresolved upstream links (retained, not inferred): ``u32``.

<a id="op-71080197084a68179c233ddf"></a>
## default

`function` · `parquet::data_type::Int96::default` · parquet 59.3.0

```rust
fn default() -> Int96
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 30], "end": [36, 37], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/data_type.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e98afd736b1b8394a4612cdf"></a>
## eq

`function` · `parquet::data_type::Int96::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Int96) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 39], "end": [36, 48], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-884f40f55359bfde5fccf809"></a>
## fmt

`function` · `parquet::data_type::Int96::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 23], "end": [36, 28], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data_type.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a74460620479475107aac387"></a>
## fmt

`function` · `parquet::data_type::Int96::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/data_type.rs:170`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53c420d4d6f8a387876accba"></a>
## from

`function` · `parquet::data_type::Int96::from` · parquet 59.3.0

```rust
fn from(buf: Vec<u32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [166, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u32"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:160`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6709ac8b06139fdecf62e61e"></a>
## max_values_iter

`function` · `parquet::data_type::Int96::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "crate::data_type::Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [679, 50], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:679`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5226fd496e19f4dfa9dc18b4"></a>
## min_values_iter

`function` · `parquet::data_type::Int96::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "crate::data_type::Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [679, 50], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:679`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15664a9f9528d435dea8d96f"></a>
## new

`function` · `parquet::data_type::Int96::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:61`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new INT96 type struct with no data set.

<a id="op-7a197bef32e701b626afe561"></a>
## partial_cmp

`function` · `parquet::data_type::Int96::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [141, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/data_type.rs:138`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87d6ae48380060e65fa8415b"></a>
## set_data

`function` · `parquet::data_type::Int96::set_data` · parquet 59.3.0

```rust
fn set_data(&mut self, elem0: u32, elem1: u32, elem2: u32)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:73`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets data for this INT96 type.

<a id="op-6f720f1866dccd63908aa1f2"></a>
## slice_as_bytes

`function` · `parquet::data_type::Int96::slice_as_bytes` · parquet 59.3.0

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 1], "end": [621, 37], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:621`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-315ac7c6d7ed55bc748df66d"></a>
## slice_as_bytes_mut

`function` · `parquet::data_type::Int96::slice_as_bytes_mut` · parquet 59.3.0

```rust
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 1], "end": [621, 37], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:621`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2320a4c3c111ec8e3966d9f7"></a>
## to_micros

`function` · `parquet::data_type::Int96::to_micros` · parquet 59.3.0

```rust
fn to_micros(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this INT96 into an i64 representing the number of MICROSECONDS since EPOCH

Will wrap around on overflow

<a id="op-408bda8b026cca36e36cace4"></a>
## to_millis

`function` · `parquet::data_type::Int96::to_millis` · parquet 59.3.0

```rust
fn to_millis(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:92`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this INT96 into an i64 representing the number of MILLISECONDS since EPOCH

Will wrap around on overflow

<a id="op-eba660719173622ad7a87a89"></a>
## to_nanos

`function` · `parquet::data_type::Int96::to_nanos` · parquet 59.3.0

```rust
fn to_nanos(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this INT96 into an i64 representing the number of NANOSECONDS since EPOCH

Will wrap around on overflow

<a id="op-3b12b43dcc4791b1d6ec88cb"></a>
## to_seconds

`function` · `parquet::data_type::Int96::to_seconds` · parquet 59.3.0

```rust
fn to_seconds(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96", "path": "Int96"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [135, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:81`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this INT96 into an i64 representing the number of SECONDS since EPOCH

Will wrap around on overflow
