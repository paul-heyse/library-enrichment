# `datafusion_datasource_json::file_format::JsonFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.file_format.JsonFormatFactory.json).

<a id="op-2b9762f0c6362d91880c7245"></a>
## JsonFormatFactory

`struct` · `datafusion_datasource_json::file_format::JsonFormatFactory` · datafusion-datasource-json 55.1.0

```rust
struct JsonFormatFactory
```

Source: `src/file_format.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Factory struct used to create [JsonFormat](../operations/datafusion_datasource_json.file_format.JsonFormat.md#op-61af6052247483fce1d8d429)

<a id="op-16242d07e4e290295986e956"></a>
## create

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::create` · datafusion-datasource-json 55.1.0

```rust
fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [114, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3792e91eada2ab9312bd94a6"></a>
## default

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::default` · datafusion-datasource-json 55.1.0

```rust
fn default() -> JsonFormatFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84d0f86836bac5e6b844dbbc"></a>
## default

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::default` · datafusion-datasource-json 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [114, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d17a98189db377474d942c94"></a>
## fmt

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::fmt` · datafusion-datasource-json 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [129, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a36be42a24716b6107263f0"></a>
## get_ext

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::get_ext` · datafusion-datasource-json 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [121, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3c853f52a9c30d14599499c"></a>
## new

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::new` · datafusion-datasource-json 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [84, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Creates an instance of [JsonFormatFactory](../operations/datafusion_datasource_json.file_format.JsonFormatFactory.md#op-2b9762f0c6362d91880c7245)

<a id="op-70e4bd29ca2fb6fe467ac347"></a>
## new_with_options

`function` · `datafusion_datasource_json::file_format::JsonFormatFactory::new_with_options` · datafusion-datasource-json 55.1.0

```rust
fn new_with_options(options: JsonOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormatFactory", "path": "JsonFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [84, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Creates an instance of [JsonFormatFactory](../operations/datafusion_datasource_json.file_format.JsonFormatFactory.md#op-2b9762f0c6362d91880c7245) with customized default options

<a id="op-07149a885b20a90c3662906b"></a>
## options

`struct_field` · `datafusion_datasource_json::file_format::JsonFormatFactory::options` · datafusion-datasource-json 55.1.0

```rust
options: Option<datafusion_common::config::JsonOptions>
```

Source: `src/file_format.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

the options carried by format factory
