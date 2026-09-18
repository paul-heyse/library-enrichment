# `arrow_avro::reader::Reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.Reader.json).

<a id="op-edbbf342c24b720967abca55"></a>
## Reader

`struct` · `arrow_avro::reader::Reader` · arrow-avro 59.3.0

```rust
struct Reader<R: BufRead>
```

Source: `src/reader/mod.rs:1334`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A high‑level Avro **Object Container File** reader.

`Reader` pulls blocks from a `BufRead` source, handles optional block compression,
and decodes them row‑by‑row into Arrow `RecordBatch` values using an internal
`Decoder`. It implements both:

* [`Iterator<Item = Result<RecordBatch, ArrowError>>`], and
* `RecordBatchReader`, guaranteeing a consistent schema across all produced batches.


Unresolved upstream links (retained, not inferred): ``Iterator<Item = Result<RecordBatch, ArrowError>>``.

<a id="op-1d9d964883020242310ac8c7"></a>
## Item

`assoc_type` · `arrow_avro::reader::Reader::Item` · arrow-avro 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1402, 1], "end": [1408, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:1403`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d35c04dd0b6035adfa300763"></a>
## avro_header

`function` · `arrow_avro::reader::Reader::avro_header` · arrow-avro 59.3.0

```rust
fn avro_header(&self) -> &Header
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1345, 1], "end": [1400, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1353`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns a reference to the parsed Avro container‑file header (magic, metadata, codec, sync).

<a id="op-3a3a1676108af4161e535990"></a>
## fmt

`function` · `arrow_avro::reader::Reader::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1333, 10], "end": [1333, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:1333`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10518a98173c100c1b916d4a"></a>
## next

`function` · `arrow_avro::reader::Reader::next` · arrow-avro 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1402, 1], "end": [1408, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:1405`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cbb9018371a5fde427c3ccb"></a>
## schema

`function` · `arrow_avro::reader::Reader::schema` · arrow-avro 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 1], "end": [1414, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/reader/mod.rs:1411`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61c0f7807d573c3370a617c3"></a>
## schema

`function` · `arrow_avro::reader::Reader::schema` · arrow-avro 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1345, 1], "end": [1400, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1348`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the Arrow schema discovered from the Avro file header (or derived via
the optional reader schema).
