# `datafusion_datasource::file_format::DefaultFileType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.DefaultFileType.json).

<a id="op-266cf80407fe6bc321faa948"></a>
## DefaultFileType

`struct` · `datafusion_datasource::file_format::DefaultFileType` · datafusion-datasource 55.1.0

```rust
struct DefaultFileType
```

Source: `src/file_format.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A container of [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973) which also implements [FileType](../operations/datafusion_common.file_options.file_type.FileType.md#op-a3f73eb11aa44031af77da32).
This enables converting a dyn FileFormat to a dyn FileType.
The former trait is a superset of the latter trait, which includes execution time
relevant methods. [FileType](../operations/datafusion_common.file_options.file_type.FileType.md#op-a3f73eb11aa44031af77da32) is only used in logical planning and only implements
the subset of methods required during logical planning.

<a id="op-bc799970359222abb2825c38"></a>
## as_any

`function` · `datafusion_datasource::file_format::DefaultFileType::as_any` · datafusion-datasource 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [255, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::FileType", "path": "FileType"}, "trait_path": "datafusion_common::file_options::file_type::FileType"}`

Source: `src/file_format.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71a357c0669a172d98ea7f99"></a>
## as_format_factory

`function` · `datafusion_datasource::file_format::DefaultFileType::as_format_factory` · datafusion-datasource 55.1.0

```rust
fn as_format_factory(&self) -> &Arc<dyn FileFormatFactory>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [249, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

get a reference to the inner [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973) struct

<a id="op-76f2f3d7527d0c11fa54ca5e"></a>
## fmt

`function` · `datafusion_datasource::file_format::DefaultFileType::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 10], "end": [232, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a66e0a74914aacf8e19ad92d"></a>
## fmt

`function` · `datafusion_datasource::file_format::DefaultFileType::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [261, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/file_format.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55d392f7c55f678960a01897"></a>
## get_ext

`function` · `datafusion_datasource::file_format::DefaultFileType::get_ext` · datafusion-datasource 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [267, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f76526285a64588c654613b"></a>
## new

`function` · `datafusion_datasource::file_format::DefaultFileType::new` · datafusion-datasource 55.1.0

```rust
fn new(file_format_factory: Arc<dyn FileFormatFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::DefaultFileType", "path": "DefaultFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [249, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Constructs a [DefaultFileType](../operations/datafusion_datasource.file_format.DefaultFileType.md#op-266cf80407fe6bc321faa948) wrapper from a [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973)
