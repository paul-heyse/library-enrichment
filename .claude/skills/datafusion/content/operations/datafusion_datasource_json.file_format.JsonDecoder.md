# `datafusion_datasource_json::file_format::JsonDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.file_format.JsonDecoder.json).

<a id="op-3b4ef8a1fdb3f88da56932d4"></a>
## JsonDecoder

`struct` · `datafusion_datasource_json::file_format::JsonDecoder` · datafusion-datasource-json 55.1.0

```rust
struct JsonDecoder
```

Source: `src/file_format.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7fa9bdf841911333c56af20"></a>
## can_flush_early

`function` · `datafusion_datasource_json::file_format::JsonDecoder::can_flush_early` · datafusion-datasource-json 55.1.0

```rust
fn can_flush_early(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonDecoder", "path": "JsonDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [617, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ed41c7bc41f0e7d93dcaa0"></a>
## decode

`function` · `datafusion_datasource_json::file_format::JsonDecoder::decode` · datafusion-datasource-json 55.1.0

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonDecoder", "path": "JsonDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [617, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-592810245c7cc4f7e52fde54"></a>
## flush

`function` · `datafusion_datasource_json::file_format::JsonDecoder::flush` · datafusion-datasource-json 55.1.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonDecoder", "path": "JsonDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [617, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7888f887f7f5abb355e3a169"></a>
## fmt

`function` · `datafusion_datasource_json::file_format::JsonDecoder::fmt` · datafusion-datasource-json 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonDecoder", "path": "JsonDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [594, 10], "end": [594, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33cb535551e662724c7f3791"></a>
## new

`function` · `datafusion_datasource_json::file_format::JsonDecoder::new` · datafusion-datasource-json 55.1.0

```rust
fn new(decoder: json::reader::Decoder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonDecoder", "path": "JsonDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [599, 1], "end": [603, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
