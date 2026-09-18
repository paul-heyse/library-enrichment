# `datafusion_datasource_avro::file_format::AvroFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_avro.file_format.AvroFormatFactory.json).

<a id="op-974af364c78f4f6b0d552f0c"></a>
## AvroFormatFactory

`struct` · `datafusion_datasource_avro::file_format::AvroFormatFactory` · datafusion-datasource-avro 55.1.0

```rust
struct AvroFormatFactory
```

Source: `src/file_format.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Factory struct used to create [`AvroFormat`](../operations/datafusion_datasource_avro.file_format.AvroFormat.md#op-fd1552b438c9a5d1a6836d40)

<a id="op-a864b1827e3616ec18574a2e"></a>
## create

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::create` · datafusion-datasource-avro 55.1.0

```rust
fn create(&self, _state: &dyn Session, _format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [67, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b7d6abe88bfe41fdfa757e5"></a>
## default

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::default` · datafusion-datasource-avro 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [67, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32a886f7f9de9185325d57c5"></a>
## default

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::default` · datafusion-datasource-avro 55.1.0

```rust
fn default() -> AvroFormatFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18922772c3d0c7aa2aa06723"></a>
## fmt

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::fmt` · datafusion-datasource-avro 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [73, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2fb9c72e617ab1cb114351b"></a>
## get_ext

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::get_ext` · datafusion-datasource-avro 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [80, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e386ed4b67df48513b09d1e3"></a>
## new

`function` · `datafusion_datasource_avro::file_format::AvroFormatFactory::new` · datafusion-datasource-avro 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::file_format::AvroFormatFactory", "path": "AvroFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [53, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Creates an instance of [`AvroFormatFactory`](../operations/datafusion_datasource_avro.file_format.AvroFormatFactory.md#op-974af364c78f4f6b0d552f0c)
