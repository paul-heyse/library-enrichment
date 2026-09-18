# `datafusion_common::file_options::parquet_writer`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.file_options.parquet_writer.json`](../model/datafusion_common.file_options.parquet_writer.json)

## parse_compression_string

`function` · `datafusion_common::file_options::parquet_writer::parse_compression_string`

```rust
fn parse_compression_string(str_setting: &str) -> Result<parquet::basic::Compression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.file_options.parquet_writer.parse_compression_string.md).


Parses datafusion.execution.parquet.compression String to a parquet::basic::Compression

---

## ParquetWriterOptions

`struct` · `datafusion_common::file_options::parquet_writer::ParquetWriterOptions`

```rust
struct ParquetWriterOptions
```

**Fields**: `writer_options`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(writer_options: WriterProperties) -> Self
fn writer_options(&self) -> &WriterProperties
```

**via `core::convert::TryFrom`**

```rust
fn try_from(parquet_table_options: &TableParquetOptions) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.file_options.parquet_writer.ParquetWriterOptions.md).


Options for writing parquet files

---
