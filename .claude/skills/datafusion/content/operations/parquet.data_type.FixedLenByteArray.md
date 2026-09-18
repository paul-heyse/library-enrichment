# `parquet::data_type::FixedLenByteArray`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.FixedLenByteArray.json).

<a id="op-35c24c1502f9d5f953e2357c"></a>
## FixedLenByteArray

`struct` · `parquet::data_type::FixedLenByteArray` · parquet 59.3.0

```rust
struct FixedLenByteArray
```

Source: `src/data_type.rs:340`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Wrapper type for performance reasons, this represents `FIXED_LEN_BYTE_ARRAY` but in all other
considerations behaves the same as `ByteArray`

# Performance notes:
This type is a little unfortunate, without it the compiler generates code that takes quite a
big hit on the CPU pipeline. Essentially the previous version stalls awaiting the result of
`T::get_physical_type() == Type::FIXED_LEN_BYTE_ARRAY`.

Its debatable if this is wanted, it is out of spec for what parquet documents as its base
types, although there are code paths in the Rust (and potentially the C++) versions that
warrant this.

With this wrapper type the compiler generates more targeted code paths matching the higher
level logical types, removing the data-hazard from all decoding and encoding paths.

<a id="op-aaadfc003ba988eb892866f9"></a>
## Item

`assoc_type` · `parquet::data_type::FixedLenByteArray::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "crate::data_type::FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [685, 53], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:684`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a4acf61c858a5869823e491"></a>
## Target

`assoc_type` · `parquet::data_type::FixedLenByteArray::Target` · parquet 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [390, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/data_type.rs:385`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a26245d7cc57d5eddc7cf56"></a>
## as_bytes

`function` · `parquet::data_type::FixedLenByteArray::as_bytes` · parquet 59.3.0

```rust
fn as_bytes(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 1], "end": [651, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}, "trait_path": "parquet::data_type::AsBytes"}`

Source: `src/data_type.rs:648`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-149599e4693b901e52bdc5fd"></a>
## as_ref

`function` · `parquet::data_type::FixedLenByteArray::as_ref` · parquet 59.3.0

```rust
fn as_ref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1326, 1], "end": [1330, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"primitive": "u8"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/data_type.rs:1327`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0fafa8f8be32d595ca6a3b3"></a>
## clone

`function` · `parquet::data_type::FixedLenByteArray::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FixedLenByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 10], "end": [339, 15], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54d7f616e9350ad2097977b5"></a>
## default

`function` · `parquet::data_type::FixedLenByteArray::default` · parquet 59.3.0

```rust
fn default() -> FixedLenByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 24], "end": [339, 31], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/data_type.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0834bca43fa89c34931d510"></a>
## deref

`function` · `parquet::data_type::FixedLenByteArray::deref` · parquet 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [390, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/data_type.rs:387`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae85a92df83c6372bda60a06"></a>
## deref_mut

`function` · `parquet::data_type::FixedLenByteArray::deref_mut` · parquet 59.3.0

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 1], "end": [396, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::ops::deref::DerefMut", "path": "DerefMut"}, "trait_path": "core::ops::deref::DerefMut"}`

Source: `src/data_type.rs:393`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51599ff65e089ef4f12a67b2"></a>
## eq

`function` · `parquet::data_type::FixedLenByteArray::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &FixedLenByteArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [346, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c3e2b3148216a772d387018"></a>
## eq

`function` · `parquet::data_type::FixedLenByteArray::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ByteArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [352, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}}], "constraints": []}}, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:349`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3036817187eed8b8c7950786"></a>
## fmt

`function` · `parquet::data_type::FixedLenByteArray::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [364, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/data_type.rs:361`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d929e024eb39b08b9f1a370d"></a>
## fmt

`function` · `parquet::data_type::FixedLenByteArray::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 17], "end": [339, 22], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data_type.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c6cd9089e9b0662b22bd256"></a>
## from

`function` · `parquet::data_type::FixedLenByteArray::from` · parquet 59.3.0

```rust
fn from(other: ByteArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [402, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:399`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d1b8764a6dea21e4459579e"></a>
## from

`function` · `parquet::data_type::FixedLenByteArray::from` · parquet 59.3.0

```rust
fn from(buf: Vec<u8>) -> FixedLenByteArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [408, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data_type.rs:405`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f66069ba9d40e469b3e379f7"></a>
## max_values_iter

`function` · `parquet::data_type::FixedLenByteArray::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "crate::data_type::FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [685, 53], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:684`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d948425cf592ebb514310be3"></a>
## min_values_iter

`function` · `parquet::data_type::FixedLenByteArray::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "crate::data_type::FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [685, 53], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexIterators", "path": "ColumnIndexIterators"}, "trait_path": "parquet::file::page_index::column_index::ColumnIndexIterators"}`

Source: `src/file/page_index/column_index.rs:684`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4148ee5cae1be9c427936627"></a>
## partial_cmp

`function` · `parquet::data_type::FixedLenByteArray::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &ByteArray) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [382, 2], "filename": "src/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArray", "path": "ByteArray"}}}], "constraints": []}}, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/data_type.rs:379`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeabe40f4ce5f14578276829"></a>
## partial_cmp

`function` · `parquet::data_type::FixedLenByteArray::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &FixedLenByteArray) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [370, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/data_type.rs:367`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-407d168db0d5439336a0f7de"></a>
## slice_as_bytes

`function` · `parquet::data_type::FixedLenByteArray::slice_as_bytes` · parquet 59.3.0

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [624, 1], "end": [624, 49], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:624`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0090f602c761dfb45123c99"></a>
## slice_as_bytes_mut

`function` · `parquet::data_type::FixedLenByteArray::slice_as_bytes_mut` · parquet 59.3.0

```rust
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArray", "path": "FixedLenByteArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [624, 1], "end": [624, 49], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::SliceAsBytes", "path": "SliceAsBytes"}, "trait_path": "parquet::data_type::SliceAsBytes"}`

Source: `src/data_type.rs:624`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
