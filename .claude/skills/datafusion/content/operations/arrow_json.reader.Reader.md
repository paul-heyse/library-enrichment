# `arrow_json::reader::Reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.Reader.json).

<a id="op-065551256893ce5f2d76aab8"></a>
## Reader

`struct` · `arrow_json::reader::Reader` · arrow-json 59.3.0

```rust
struct Reader<R>
```

Source: `src/reader/mod.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Reads JSON data with a known schema directly into arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

Lines consisting solely of ASCII whitespace are ignored

<a id="op-175bc2d967a8ceb7f04654e9"></a>
## Item

`assoc_type` · `arrow_json::reader::Reader::Item` · arrow-json 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 1], "end": [398, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06d9488424d7487a7cd15e7c"></a>
## fmt

`function` · `arrow_json::reader::Reader::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [370, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ea27bfba0e886f12853a85e"></a>
## next

`function` · `arrow_json::reader::Reader::next` · arrow-json 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 1], "end": [398, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:395`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0ce95b17329228aa7d44e51"></a>
## schema

`function` · `arrow_json::reader::Reader::schema` · arrow-json 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::Reader", "path": "Reader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [404, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/reader/mod.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
