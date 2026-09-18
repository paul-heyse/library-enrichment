# `arrow_avro::reader::header::HeaderInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.header.HeaderInfo.json).

<a id="op-cfe8c3dfb2978eb03d7adba6"></a>
## HeaderInfo

`struct` · `arrow_avro::reader::header::HeaderInfo` · arrow-avro 59.3.0

```rust
struct HeaderInfo
```

Source: `src/reader/header.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Header information for an Avro OCF file.

The header can be parsed once and shared to construct multiple readers
for the same file, and so this struct is designed to be cheaply clonable.

<a id="op-9d4c3969d3b1b6aeff9e70a2"></a>
## clone

`function` · `arrow_avro::reader::header::HeaderInfo::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> HeaderInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::header::HeaderInfo", "path": "HeaderInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 10], "end": [138, 15], "filename": "src/reader/header.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/reader/header.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2deebd7f1e3c66f1581cad"></a>
## compression

`function` · `arrow_avro::reader::header::HeaderInfo::compression` · arrow-avro 59.3.0

```rust
fn compression(&self) -> Result<Option<CompressionCodec>, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::header::HeaderInfo", "path": "HeaderInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [186, 2], "filename": "src/reader/header.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/header.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the [`CompressionCodec`](../operations/arrow_avro.compression.CompressionCodec.md#op-ba58cf6b7543b446614d7124) if any

<a id="op-1f236595225b17cfa4b948e5"></a>
## header_len

`function` · `arrow_avro::reader::header::HeaderInfo::header_len` · arrow-avro 59.3.0

```rust
fn header_len(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::header::HeaderInfo", "path": "HeaderInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [186, 2], "filename": "src/reader/header.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/header.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the length of the header in bytes.

<a id="op-2fa602cfd99021f2c5400f62"></a>
## sync

`function` · `arrow_avro::reader::header::HeaderInfo::sync` · arrow-avro 59.3.0

```rust
fn sync(&self) -> [u8; 16]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::header::HeaderInfo", "path": "HeaderInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [186, 2], "filename": "src/reader/header.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/header.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the sync token for this file.

<a id="op-44bf9a69dac331174adb9f92"></a>
## writer_schema

`function` · `arrow_avro::reader::header::HeaderInfo::writer_schema` · arrow-avro 59.3.0

```rust
fn writer_schema(&self) -> Result<AvroSchema, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::header::HeaderInfo", "path": "HeaderInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [186, 2], "filename": "src/reader/header.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/header.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the writer schema for this file.
