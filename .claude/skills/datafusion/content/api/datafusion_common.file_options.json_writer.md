# `datafusion_common::file_options::json_writer`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.file_options.json_writer.json`](../model/datafusion_common.file_options.json_writer.json)

## JsonWriterOptions

`struct` · `datafusion_common::file_options::json_writer::JsonWriterOptions`

```rust
struct JsonWriterOptions
```

**Fields**: `compression`, `compression_level`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(compression: CompressionTypeVariant) -> Self
fn new_with_level(compression: CompressionTypeVariant, compression_level: u32) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &JsonOptions) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.file_options.json_writer.JsonWriterOptions.md).


Options for writing JSON files

---
