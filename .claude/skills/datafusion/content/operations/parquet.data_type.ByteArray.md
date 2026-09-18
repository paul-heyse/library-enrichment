# `parquet::data_type::ByteArray`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.ByteArray.json).

<a id="op-fff27d3428efa4abd643a59e"></a>
## ByteArray

`struct` · `parquet::data_type::ByteArray` · parquet 59.3.0

```rust
struct ByteArray
```

Source: `src/data_type.rs:178`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Rust representation for BYTE_ARRAY and FIXED_LEN_BYTE_ARRAY Parquet physical types.
Value is backed by a byte buffer.

<a id="op-6691cda277f95f27c09e4f00"></a>
## Item

`assoc_type` · `parquet::data_type::ByteArray::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "crate::data_type::ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [682, 1], "end": [683, 45], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eefdbfd9c085f73a46ed6fd"></a>
## as_bytes

`function` · `parquet::data_type::ByteArray::as_bytes` · parquet 59.3.0

```rust
fn as_bytes(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [645, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}, "trait_path": "parquet::data_type::AsBytes"}`

Source: `src/data_type.rs:642`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c0fe1e2fe7edcfb1a660b32"></a>
## as_ref

`function` · `parquet::data_type::ByteArray::as_ref` · parquet 59.3.0

```rust
fn as_ref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1320, 1], "end": [1324, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"primitive": "u8"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/data_type.rs:1321`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-566ad19aaf01ff43c5e2d9e0"></a>
## as_utf8

`function` · `parquet::data_type::ByteArray::as_utf8` · parquet 59.3.0

```rust
fn as_utf8(&self) -> Result<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:259`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to convert the byte array to a utf8 slice

<a id="op-071363065bba1b84242cbf4d"></a>
## clone

`function` · `parquet::data_type::ByteArray::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 10], "end": [177, 15], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7c634a865d7491d56245180"></a>
## data

`function` · `parquet::data_type::ByteArray::data` · parquet 59.3.0

```rust
fn data(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:234`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of data.

<a id="op-8ee93604f084c444517e3e14"></a>
## default

`function` · `parquet::data_type::ByteArray::default` · parquet 59.3.0

```rust
fn default() -> ByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 17], "end": [177, 24], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/data_type.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2976203ca97d1b3a0022160f"></a>
## eq

`function` · `parquet::data_type::ByteArray::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &FixedLenByteArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [358, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}}], "constraints": []}}, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:355`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bd3dc189a7e20f7fc32d98e"></a>
## eq

`function` · `parquet::data_type::ByteArray::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ByteArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [316, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:309`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0454661acd244dea1a96c1b1"></a>
## fmt

`function` · `parquet::data_type::ByteArray::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [192, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data_type.rs:184`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf1cc2e8caa406ccedfab17b"></a>
## fmt

`function` · `parquet::data_type::ByteArray::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [318, 1], "end": [322, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/data_type.rs:319`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32e085a64ebf2068ee837ab5"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(b: &'a [u8]) -> ByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [276, 1], "end": [284, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:277`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40e2ba5dd05d4f882550a573"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(value: f16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 1], "end": [306, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "half::binary16::f16", "path": "f16"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bb6452268d557cc33fdbc71"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(s: &'a str) -> ByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:287`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-662ac9b5a6728eb79abdc456"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(value: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [300, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:297`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f803fb2eb8f98c02b6448b1"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(buf: Vec<u8>) -> ByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [274, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:269`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a44da0159c9544af232f8f9"></a>
## from

`function` · `parquet::data_type::ByteArray::from` · parquet 59.3.0

```rust
fn from(other: FixedLenByteArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [414, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:411`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ec789749554c859c4a31ec"></a>
## is_empty

`function` · `parquet::data_type::ByteArray::is_empty` · parquet 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:228`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Checks if the underlying buffer is empty.

<a id="op-131e79b18652472ab2e56af8"></a>
## len

`function` · `parquet::data_type::ByteArray::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:221`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets length of the underlying byte buffer.

<a id="op-acf08e38ec91b649aaa70ce4"></a>
## max_values_iter

`function` · `parquet::data_type::ByteArray::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "crate::data_type::ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [682, 1], "end": [683, 45], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd208431423aad908d9b9d04"></a>
## min_values_iter

`function` · `parquet::data_type::ByteArray::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "crate::data_type::ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [682, 1], "end": [683, 45], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbf51f7c118d22007081866f"></a>
## new

`function` · `parquet::data_type::ByteArray::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:215`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new byte array with no data set.

<a id="op-069ae3f7d7595461b3b1c283"></a>
## partial_cmp

`function` · `parquet::data_type::ByteArray::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &FixedLenByteArray) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 1], "end": [376, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}}], "constraints": []}}, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/data_type.rs:373`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b849e677f0def75c208636e5"></a>
## partial_cmp

`function` · `parquet::data_type::ByteArray::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &ByteArray) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [210, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/data_type.rs:195`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08dcb317d4347f351c777341"></a>
## set_data

`function` · `parquet::data_type::ByteArray::set_data` · parquet 59.3.0

```rust
fn set_data(&mut self, data: Bytes)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:243`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set data from another byte buffer.

<a id="op-c8e1a61900be6c95a390d79b"></a>
## slice

`function` · `parquet::data_type::ByteArray::slice` · parquet 59.3.0

```rust
fn slice(&self, start: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [266, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `ByteArray` instance with slice of values for a data.

<a id="op-b21f61bdf6d6293863eb981e"></a>
## slice_as_bytes

`function` · `parquet::data_type::ByteArray::slice_as_bytes` · parquet 59.3.0

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 1], "end": [623, 41], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:623`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1b32a4de836e28b29321e69"></a>
## slice_as_bytes_mut

`function` · `parquet::data_type::ByteArray::slice_as_bytes_mut` · parquet 59.3.0

```rust
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 1], "end": [623, 41], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:623`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
