# `parquet::basic::CompressionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.CompressionCodec.json).

<a id="op-c6c3bc2803e50d819f81d88f"></a>
## CompressionCodec

`enum` · `parquet::basic::CompressionCodec` · parquet 59.3.0

```rust
enum CompressionCodec
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Supported compression algorithms.

Codecs added in format version X.Y can be read by readers based on X.Y and later.
Codec support may vary between readers based on the format version and
libraries available at runtime.

See [Compression.md] for a detailed specification of these algorithms.

[Compression.md]: https://github.com/apache/parquet-format/blob/master/Compression.md

<a id="op-3eb7de678f2bfd917a358c28"></a>
## BROTLI

`variant` · `parquet::basic::CompressionCodec::BROTLI` · parquet 59.3.0

```rust
BROTLI
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a40c9d48ec387ac24bbaa34"></a>
## GZIP

`variant` · `parquet::basic::CompressionCodec::GZIP` · parquet 59.3.0

```rust
GZIP
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-020097e21ae868b0eae7c5e8"></a>
## LZ4

`variant` · `parquet::basic::CompressionCodec::LZ4` · parquet 59.3.0

```rust
LZ4
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2092fb19616902a3d265973c"></a>
## LZ4_RAW

`variant` · `parquet::basic::CompressionCodec::LZ4_RAW` · parquet 59.3.0

```rust
LZ4_RAW
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5733aacab86f549e513ede06"></a>
## LZO

`variant` · `parquet::basic::CompressionCodec::LZO` · parquet 59.3.0

```rust
LZO
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa859aec453d16ad4e0f8038"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::CompressionCodec::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-8474b3bd31d4f2feecd50e65"></a>
## SNAPPY

`variant` · `parquet::basic::CompressionCodec::SNAPPY` · parquet 59.3.0

```rust
SNAPPY
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb6610480b811ab523126e44"></a>
## UNCOMPRESSED

`variant` · `parquet::basic::CompressionCodec::UNCOMPRESSED` · parquet 59.3.0

```rust
UNCOMPRESSED
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05b13ad705337a8287e9f32"></a>
## VARIANTS

`assoc_const` · `parquet::basic::CompressionCodec::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-394d36ae9e8e6fb4ab8297ff"></a>
## ZSTD

`variant` · `parquet::basic::CompressionCodec::ZSTD` · parquet 59.3.0

```rust
ZSTD
```

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c57eb496cf6eb1b27f3215b"></a>
## clone

`function` · `parquet::basic::CompressionCodec::clone` · parquet 59.3.0

```rust
fn clone(&self) -> CompressionCodec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e66836fe6a12fe7604fac350"></a>
## cmp

`function` · `parquet::basic::CompressionCodec::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &CompressionCodec) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d370b16fe6855fa01d6e9ee"></a>
## eq

`function` · `parquet::basic::CompressionCodec::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &CompressionCodec) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a7fe18f7c3a5edc88b75d35"></a>
## fmt

`function` · `parquet::basic::CompressionCodec::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2e264add73daf633d97e502"></a>
## fmt

`function` · `parquet::basic::CompressionCodec::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff85302484c29ad514a48353"></a>
## from

`function` · `parquet::basic::CompressionCodec::from` · parquet 59.3.0

```rust
fn from(value: Compression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [696, 1], "end": [709, 2], "filename": "src/basic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/basic.rs:697`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d92d7796850048f0c600e303"></a>
## hash

`function` · `parquet::basic::CompressionCodec::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a17adae42e1a04a08ef4083"></a>
## partial_cmp

`function` · `parquet::basic::CompressionCodec::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &CompressionCodec) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [638, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
