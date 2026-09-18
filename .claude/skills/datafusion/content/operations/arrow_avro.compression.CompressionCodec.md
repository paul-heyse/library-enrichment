# `arrow_avro::compression::CompressionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.compression.CompressionCodec.json).

<a id="op-ba58cf6b7543b446614d7124"></a>
## CompressionCodec

`enum` · `arrow_avro::compression::CompressionCodec` · arrow-avro 59.3.0

```rust
enum CompressionCodec
```

Source: `src/compression.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Supported compression codecs for Avro data

Avro supports multiple compression formats for data blocks.
This enum represents the compression codecs available in this implementation.

<a id="op-6a76aede4840e73b272332c7"></a>
## Bzip2

`variant` · `arrow_avro::compression::CompressionCodec::Bzip2` · arrow-avro 59.3.0

```rust
Bzip2
```

Source: `src/compression.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Bzip2 compression

<a id="op-7ce260d29abb70140dc81b7d"></a>
## Deflate

`variant` · `arrow_avro::compression::CompressionCodec::Deflate` · arrow-avro 59.3.0

```rust
Deflate
```

Source: `src/compression.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Deflate compression (RFC 1951)

<a id="op-8609e1a606036d820d056b56"></a>
## Snappy

`variant` · `arrow_avro::compression::CompressionCodec::Snappy` · arrow-avro 59.3.0

```rust
Snappy
```

Source: `src/compression.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Snappy compression

<a id="op-3ede01657ff8c534e3f946ab"></a>
## Xz

`variant` · `arrow_avro::compression::CompressionCodec::Xz` · arrow-avro 59.3.0

```rust
Xz
```

Source: `src/compression.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Xz compression

<a id="op-4363fd19104a1b7f9dd132b2"></a>
## ZStandard

`variant` · `arrow_avro::compression::CompressionCodec::ZStandard` · arrow-avro 59.3.0

```rust
ZStandard
```

Source: `src/compression.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

ZStandard compression

<a id="op-a1da8b2bc30016c921113634"></a>
## clone

`function` · `arrow_avro::compression::CompressionCodec::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> CompressionCodec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::compression::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 23], "end": [31, 28], "filename": "src/compression.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/compression.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afb81e81b8981bb4dae7a940"></a>
## eq

`function` · `arrow_avro::compression::CompressionCodec::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &CompressionCodec) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::compression::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 34], "end": [31, 43], "filename": "src/compression.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/compression.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75a09940ee5e9ce3e7a09fd4"></a>
## fmt

`function` · `arrow_avro::compression::CompressionCodec::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::compression::CompressionCodec", "path": "CompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/compression.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/compression.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
