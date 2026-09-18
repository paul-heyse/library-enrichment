# `datafusion_datasource_json::file_format::JsonSerializer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.file_format.JsonSerializer.json).

<a id="op-4e3a7ba5f9da6c0923dce23c"></a>
## JsonSerializer

`struct` · `datafusion_datasource_json::file_format::JsonSerializer` · datafusion-datasource-json 55.1.0

```rust
struct JsonSerializer
```

Source: `src/file_format.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Define a struct for serializing Json records to a stream

<a id="op-8922e35db475f8fec770aab4"></a>
## default

`function` · `datafusion_datasource_json::file_format::JsonSerializer::default` · datafusion-datasource-json 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSerializer", "path": "JsonSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [383, 1], "end": [387, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebddba093cda4cea05c8580b"></a>
## new

`function` · `datafusion_datasource_json::file_format::JsonSerializer::new` · datafusion-datasource-json 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSerializer", "path": "JsonSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 1], "end": [397, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Constructor for the JsonSerializer object

<a id="op-990f856a80ce7b362231ffa0"></a>
## serialize

`function` · `datafusion_datasource_json::file_format::JsonSerializer::serialize` · datafusion-datasource-json 55.1.0

```rust
fn serialize(&self, batch: RecordBatch, _initial: bool) -> Result<Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSerializer", "path": "JsonSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [399, 1], "end": [406, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::write::BatchSerializer", "path": "BatchSerializer"}, "trait_path": "datafusion_datasource::write::BatchSerializer"}`

Source: `src/file_format.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
