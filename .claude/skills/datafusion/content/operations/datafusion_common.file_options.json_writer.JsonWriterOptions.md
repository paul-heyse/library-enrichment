# `datafusion_common::file_options::json_writer::JsonWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.file_options.json_writer.JsonWriterOptions.json).

<a id="op-d561509fbe97097cd98ad6b5"></a>
## JsonWriterOptions

`struct` · `datafusion_common::file_options::json_writer::JsonWriterOptions` · datafusion-common 55.1.0

```rust
struct JsonWriterOptions
```

Source: `src/file_options/json_writer.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for writing JSON files

<a id="op-7aa006428c826d4c434caae0"></a>
## Error

`assoc_type` · `datafusion_common::file_options::json_writer::JsonWriterOptions::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [62, 2], "filename": "src/file_options/json_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/json_writer.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4529fb8a7e5e664839c785f0"></a>
## clone

`function` · `datafusion_common::file_options::json_writer::JsonWriterOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> JsonWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/file_options/json_writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_options/json_writer.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b8b0da6b3d8366cbda132ba"></a>
## compression

`struct_field` · `datafusion_common::file_options::json_writer::JsonWriterOptions::compression` · datafusion-common 55.1.0

```rust
compression: parsers::CompressionTypeVariant
```

Source: `src/file_options/json_writer.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1821229b7e679212d90d416b"></a>
## compression_level

`struct_field` · `datafusion_common::file_options::json_writer::JsonWriterOptions::compression_level` · datafusion-common 55.1.0

```rust
compression_level: Option<u32>
```

Source: `src/file_options/json_writer.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2431e4d956d56f7d4579119f"></a>
## fmt

`function` · `datafusion_common::file_options::json_writer::JsonWriterOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 17], "end": [27, 22], "filename": "src/file_options/json_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_options/json_writer.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7ea7aee7512619a35e7d063"></a>
## new

`function` · `datafusion_common::file_options::json_writer::JsonWriterOptions::new` · datafusion-common 55.1.0

```rust
fn new(compression: CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [51, 2], "filename": "src/file_options/json_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/json_writer.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fcc0125e0773dd318afa110"></a>
## new_with_level

`function` · `datafusion_common::file_options::json_writer::JsonWriterOptions::new_with_level` · datafusion-common 55.1.0

```rust
fn new_with_level(compression: CompressionTypeVariant, compression_level: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [51, 2], "filename": "src/file_options/json_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/json_writer.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new `JsonWriterOptions` with the specified compression and level.

<a id="op-f6813ef66b7e0c9f4f5a77f6"></a>
## try_from

`function` · `datafusion_common::file_options::json_writer::JsonWriterOptions::try_from` · datafusion-common 55.1.0

```rust
fn try_from(value: &JsonOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [62, 2], "filename": "src/file_options/json_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/json_writer.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
