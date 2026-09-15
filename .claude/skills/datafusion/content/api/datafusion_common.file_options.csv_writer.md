# `datafusion_common::file_options::csv_writer`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.file_options.csv_writer.json`](../model/datafusion_common.file_options.csv_writer.json)

## CsvWriterOptions

`struct` · `datafusion_common::file_options::csv_writer::CsvWriterOptions`

```rust
struct CsvWriterOptions
```

**Fields**: `writer_options`, `compression`, `compression_level`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(writer_options: WriterBuilder, compression: CompressionTypeVariant) -> Self
fn new_with_level(writer_options: WriterBuilder, compression: CompressionTypeVariant, compression_level: u32) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &CsvOptions) -> Result<Self>
```

Options for writing CSV files

---
