# `datafusion_common::file_options::parquet_writer::ParquetWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.file_options.parquet_writer.ParquetWriterOptions.json).

<a id="op-fd5f1139f886cc3d85a87fe4"></a>
## ParquetWriterOptions

`struct` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions` · datafusion-common 55.1.0

```rust
struct ParquetWriterOptions
```

Source: `src/file_options/parquet_writer.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for writing parquet files

<a id="op-533b3fc5d9842cd684f87929"></a>
## Error

`assoc_type` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [82, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/parquet_writer.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c2ed288b05558662072d5d6"></a>
## clone

`function` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ParquetWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/file_options/parquet_writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_options/parquet_writer.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b04420d85e3d3174c1f210b0"></a>
## fmt

`function` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 22], "filename": "src/file_options/parquet_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_options/parquet_writer.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adb9cf91a4edf953d3f6e34d"></a>
## new

`function` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::new` · datafusion-common 55.1.0

```rust
fn new(writer_options: WriterProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [53, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/parquet_writer.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422cf661cd5f377203b9ee6e"></a>
## try_from

`function` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::try_from` · datafusion-common 55.1.0

```rust
fn try_from(parquet_table_options: &TableParquetOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [82, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/parquet_writer.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ba36e05cb54be2b83547f32"></a>
## writer_options

`function` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::writer_options` · datafusion-common 55.1.0

```rust
fn writer_options(&self) -> &WriterProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::parquet_writer::ParquetWriterOptions", "path": "ParquetWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [59, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/parquet_writer.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b50fa9507a8a1470cb02750"></a>
## writer_options

`struct_field` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions::writer_options` · datafusion-common 55.1.0

```rust
writer_options: parquet::file::properties::WriterProperties
```

Source: `src/file_options/parquet_writer.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

parquet-rs writer properties
