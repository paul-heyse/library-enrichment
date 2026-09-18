# `datafusion_datasource_arrow::file_format::ArrowFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_arrow.file_format.ArrowFormatFactory.json).

<a id="op-fc7b3927d6b278d8c7ec9554"></a>
## ArrowFormatFactory

`struct` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory` · datafusion-datasource-arrow 55.1.0

```rust
struct ArrowFormatFactory
```

Source: `src/file_format.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Factory struct used to create [`ArrowFormat`](../operations/datafusion_datasource_arrow.file_format.ArrowFormat.md#op-b1ded2a94ee7e92e27d6b1de)

<a id="op-ad5d51e4aa4fac3be2f91234"></a>
## create

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::create` · datafusion-datasource-arrow 55.1.0

```rust
fn create(&self, _state: &dyn Session, _format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [100, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-490a566283c6552eef3be7dd"></a>
## default

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::default` · datafusion-datasource-arrow 55.1.0

```rust
fn default() -> ArrowFormatFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 10], "end": [78, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca109c4f38d1db64c2144dfc"></a>
## default

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::default` · datafusion-datasource-arrow 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [100, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d021a516a1a3a2b25eb7a289"></a>
## fmt

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::fmt` · datafusion-datasource-arrow 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 19], "end": [78, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c488af5279919518543de98"></a>
## get_ext

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::get_ext` · datafusion-datasource-arrow 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [107, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41064ea18e2806cce4922c40"></a>
## new

`function` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory::new` · datafusion-datasource-arrow 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormatFactory", "path": "ArrowFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [86, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Creates an instance of [ArrowFormatFactory](../operations/datafusion_datasource_arrow.file_format.ArrowFormatFactory.md#op-fc7b3927d6b278d8c7ec9554)
