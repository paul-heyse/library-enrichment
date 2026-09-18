# `datafusion_common::file_options::csv_writer::CsvWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.file_options.csv_writer.CsvWriterOptions.json).

<a id="op-7679413a9ce6955e786f721d"></a>
## CsvWriterOptions

`struct` · `datafusion_common::file_options::csv_writer::CsvWriterOptions` · datafusion-common 55.1.0

```rust
struct CsvWriterOptions
```

Source: `src/file_options/csv_writer.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for writing CSV files

<a id="op-2b6da5cd99a185f2254a40ff"></a>
## Error

`assoc_type` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [110, 2], "filename": "src/file_options/csv_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/csv_writer.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e18c6741e4bedafd12aebbab"></a>
## clone

`function` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> CsvWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/file_options/csv_writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_options/csv_writer.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52fc35e9634e1396b8e68761"></a>
## compression

`struct_field` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::compression` · datafusion-common 55.1.0

```rust
compression: parsers::CompressionTypeVariant
```

Source: `src/file_options/csv_writer.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compression to apply after ArrowWriter serializes RecordBatches.
This compression is applied by DataFusion not the ArrowWriter itself.

<a id="op-1dbedfef21849dbad1007485"></a>
## compression_level

`struct_field` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::compression_level` · datafusion-common 55.1.0

```rust
compression_level: Option<u32>
```

Source: `src/file_options/csv_writer.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compression level for the output file.

<a id="op-b63a931c0e7f7339afd5271d"></a>
## fmt

`function` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 17], "end": [27, 22], "filename": "src/file_options/csv_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_options/csv_writer.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-365caafadf2359cac53adaaf"></a>
## new

`function` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::new` · datafusion-common 55.1.0

```rust
fn new(writer_options: WriterBuilder, compression: CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [62, 2], "filename": "src/file_options/csv_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/csv_writer.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9404c54fdaf514f5ff81169"></a>
## new_with_level

`function` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::new_with_level` · datafusion-common 55.1.0

```rust
fn new_with_level(writer_options: WriterBuilder, compression: CompressionTypeVariant, compression_level: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [62, 2], "filename": "src/file_options/csv_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/csv_writer.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new `CsvWriterOptions` with the specified compression level.

<a id="op-9de13f1aa1698f0faf4cbd44"></a>
## try_from

`function` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::try_from` · datafusion-common 55.1.0

```rust
fn try_from(value: &CsvOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [110, 2], "filename": "src/file_options/csv_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_options/csv_writer.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f6fd73d8b106e2822628f33"></a>
## writer_options

`struct_field` · `datafusion_common::file_options::csv_writer::CsvWriterOptions::writer_options` · datafusion-common 55.1.0

```rust
writer_options: arrow::csv::WriterBuilder
```

Source: `src/file_options/csv_writer.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Struct from the arrow crate which contains all csv writing related settings
