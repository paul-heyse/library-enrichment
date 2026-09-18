# `datafusion_datasource_avro::file_format::AvroFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_avro.file_format.AvroFormat.json).

<a id="op-fd1552b438c9a5d1a6836d40"></a>
## AvroFormat

`struct` · `datafusion_datasource_avro::file_format::AvroFormat` · datafusion-datasource-avro 55.1.0

```rust
struct AvroFormat
```

Source: `src/file_format.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Avro [`FileFormat`](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) implementation.

<a id="op-0d7a6a2e40e033dc7d02f186"></a>
## compression_type

`function` · `datafusion_datasource_avro::file_format::AvroFormat::compression_type` · datafusion-datasource-avro 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acbed49ab647f242e7a95f54"></a>
## create_physical_plan

`function` · `datafusion_datasource_avro::file_format::AvroFormat::create_physical_plan` · datafusion-datasource-avro 55.1.0

```rust
async fn create_physical_plan(&self, _state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ecdebdad1c8c6516458e5e5"></a>
## default

`function` · `datafusion_datasource_avro::file_format::AvroFormat::default` · datafusion-datasource-avro 55.1.0

```rust
fn default() -> AvroFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 10], "end": [83, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e328be1da29fb75caed98618"></a>
## file_source

`function` · `datafusion_datasource_avro::file_format::AvroFormat::file_source` · datafusion-datasource-avro 55.1.0

```rust
fn file_source(&self, table_schema: datafusion_datasource::TableSchema) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1be60cb9b60578ccf84718e"></a>
## fmt

`function` · `datafusion_datasource_avro::file_format::AvroFormat::fmt` · datafusion-datasource-avro 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 19], "end": [83, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b2192579c69d642725ccfef"></a>
## get_ext

`function` · `datafusion_datasource_avro::file_format::AvroFormat::get_ext` · datafusion-datasource-avro 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9552105e47242ad630c7bcac"></a>
## get_ext_with_compression

`function` · `datafusion_datasource_avro::file_format::AvroFormat::get_ext_with_compression` · datafusion-datasource-avro 55.1.0

```rust
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f2eee6391b875a2684e31c"></a>
## infer_schema

`function` · `datafusion_datasource_avro::file_format::AvroFormat::infer_schema` · datafusion-datasource-avro 55.1.0

```rust
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dde32f160e4cea228632d69d"></a>
## infer_stats

`function` · `datafusion_datasource_avro::file_format::AvroFormat::infer_stats` · datafusion-datasource-avro 55.1.0

```rust
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormat", "path": "AvroFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [156, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
