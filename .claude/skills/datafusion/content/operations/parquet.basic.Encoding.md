# `parquet::basic::Encoding`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.Encoding.json).

<a id="op-ea6de82a506bce95510cae40"></a>
## Encoding

`enum` · `parquet::basic::Encoding` · parquet 59.3.0

```rust
enum Encoding
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encodings supported by Parquet.

Not all encodings are valid for all types. These enums are also used to specify the
encoding of definition and repetition levels.

By default this crate uses [Encoding::PLAIN](../operations/parquet.basic.Encoding.md#op-e858206254caa12d0f158017), [Encoding::RLE](../operations/parquet.basic.Encoding.md#op-8008d574fc0122a1a11629ec), and [Encoding::RLE_DICTIONARY](../operations/parquet.basic.Encoding.md#op-8ab3637467eca44aabb9f36a).
These provide very good encode and decode performance, whilst yielding reasonable storage
efficiency and being supported by all major parquet readers.

The delta encodings are also supported and will be used if a newer [WriterVersion] is
configured, however, it should be noted that these sacrifice encode and decode performance for
improved storage efficiency. This performance regression is particularly pronounced in the case
of record skipping as occurs during predicate push-down. It is recommended users assess the
performance impact when evaluating these encodings.

[WriterVersion]: crate::file::properties::WriterVersion

<a id="op-6ad870ac059fda9770bda759"></a>
## BIT_PACKED

`variant` · `parquet::basic::Encoding::BIT_PACKED` · parquet 59.3.0

```rust
BIT_PACKED
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

**Deprecated** Bit-packed encoding.

This can only be used if the data has a known max width.
Usable for definition/repetition levels encoding.

There are compatibility issues with files using this encoding.
The parquet standard specifies the bits to be packed starting from the
most-significant bit, several implementations do not follow this bit order.
Several other implementations also have issues reading this encoding
because of incorrect assumptions about the length of the encoded data.

The RLE/bit-packing hybrid is more cpu and memory efficient and should be used instead.

<a id="op-749dc6d06e4b6e7b66eb866d"></a>
## BYTE_STREAM_SPLIT

`variant` · `parquet::basic::Encoding::BYTE_STREAM_SPLIT` · parquet 59.3.0

```rust
BYTE_STREAM_SPLIT
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encoding for fixed-width data.

K byte-streams are created where K is the size in bytes of the data type.
The individual bytes of a value are scattered to the corresponding stream and
the streams are concatenated.
This itself does not reduce the size of the data but can lead to better compression
afterwards. Note that the use of this encoding with FIXED_LEN_BYTE_ARRAY(N) data may
perform poorly for large values of N.

<a id="op-eda2998f57cc4a86e9195e0a"></a>
## DELTA_BINARY_PACKED

`variant` · `parquet::basic::Encoding::DELTA_BINARY_PACKED` · parquet 59.3.0

```rust
DELTA_BINARY_PACKED
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Delta encoding for integers, either INT32 or INT64.

Works best on sorted data.

<a id="op-478d9257df0cd1b55d6efbc7"></a>
## DELTA_BYTE_ARRAY

`variant` · `parquet::basic::Encoding::DELTA_BYTE_ARRAY` · parquet 59.3.0

```rust
DELTA_BYTE_ARRAY
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Incremental encoding for byte arrays.

Prefix lengths are encoded using DELTA_BINARY_PACKED encoding.
Suffixes are stored using DELTA_LENGTH_BYTE_ARRAY encoding.

<a id="op-dcf672d0f7cc75fbd6a74cb5"></a>
## DELTA_LENGTH_BYTE_ARRAY

`variant` · `parquet::basic::Encoding::DELTA_LENGTH_BYTE_ARRAY` · parquet 59.3.0

```rust
DELTA_LENGTH_BYTE_ARRAY
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encoding for byte arrays to separate the length values and the data.

The lengths are encoded using DELTA_BINARY_PACKED encoding.

<a id="op-cd3f52a766f06b5b1187f744"></a>
## Err

`assoc_type` · `parquet::basic::Encoding::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [475, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:456`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e537a8bbee69a919f72a90d"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::Encoding::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-e858206254caa12d0f158017"></a>
## PLAIN

`variant` · `parquet::basic::Encoding::PLAIN` · parquet 59.3.0

```rust
PLAIN
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Default encoding.
- BOOLEAN - 1 bit per value. 0 is false; 1 is true.
- INT32 - 4 bytes per value.  Stored as little-endian.
- INT64 - 8 bytes per value.  Stored as little-endian.
- FLOAT - 4 bytes per value.  IEEE. Stored as little-endian.
- DOUBLE - 8 bytes per value.  IEEE. Stored as little-endian.
- BYTE_ARRAY - 4 byte length stored as little endian, followed by bytes.
- FIXED_LEN_BYTE_ARRAY - Just the bytes.

<a id="op-aac008089d5300ce84c53de0"></a>
## PLAIN_DICTIONARY

`variant` · `parquet::basic::Encoding::PLAIN_DICTIONARY` · parquet 59.3.0

```rust
PLAIN_DICTIONARY
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

**Deprecated** dictionary encoding.

The values in the dictionary are encoded using PLAIN encoding.
Since it is deprecated, RLE_DICTIONARY encoding is used for a data page, and
PLAIN encoding is used for dictionary page.

<a id="op-8008d574fc0122a1a11629ec"></a>
## RLE

`variant` · `parquet::basic::Encoding::RLE` · parquet 59.3.0

```rust
RLE
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Group packed run length encoding.

Usable for definition/repetition levels encoding and boolean values.

<a id="op-8ab3637467eca44aabb9f36a"></a>
## RLE_DICTIONARY

`variant` · `parquet::basic::Encoding::RLE_DICTIONARY` · parquet 59.3.0

```rust
RLE_DICTIONARY
```

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Dictionary encoding.

The ids are encoded using the RLE encoding.

<a id="op-47442f8fc236c29b74a60aa5"></a>
## VARIANTS

`assoc_const` · `parquet::basic::Encoding::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-57a23277385f819116cb98bb"></a>
## clone

`function` · `parquet::basic::Encoding::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Encoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0fddfb72aedf97025d45548"></a>
## cmp

`function` · `parquet::basic::Encoding::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &Encoding) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b71dea1e86d52f22e12d743"></a>
## eq

`function` · `parquet::basic::Encoding::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Encoding) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2b04b5b4ae6a62fbba647b1"></a>
## fmt

`function` · `parquet::basic::Encoding::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2871e8b6445c3a3655ad626"></a>
## fmt

`function` · `parquet::basic::Encoding::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2653705287802953f322fd2"></a>
## from_str

`function` · `parquet::basic::Encoding::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [475, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:458`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ddf0d08ceb8145a20a9194"></a>
## hash

`function` · `parquet::basic::Encoding::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abfb890fd13826bd1e4e9dc5"></a>
## partial_cmp

`function` · `parquet::basic::Encoding::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &Encoding) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Encoding", "path": "Encoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [453, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
