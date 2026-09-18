# `datafusion_datasource::file_compression_type`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.file_compression_type.json`](../model/datafusion_datasource.file_compression_type.json)

## FileCompressionType

`struct` · `datafusion_datasource::file_compression_type::FileCompressionType`

Also reachable as `datafusion::datasource::file_format::file_compression_type::FileCompressionType`

```rust
struct FileCompressionType
```

**Implements**: `core::convert::From`, `core::str::traits::FromStr`, `datafusion_common::file_options::file_type::GetExt`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn convert_async_writer(&self, w: BufWriter) -> Result<Box<dyn AsyncWrite + Send + Unpin>>
fn convert_async_writer_with_level(&self, w: BufWriter, compression_level: Option<u32>) -> Result<Box<dyn AsyncWrite + Send + Unpin>>
fn convert_read<T: std::io::Read + Send + 'static>(&self, r: T) -> Result<Box<dyn std::io::Read + Send>>
fn convert_stream<'a>(&self, s: BoxStream<'a, Result<Bytes>>) -> Result<BoxStream<'a, Result<Bytes>>>
fn convert_to_compress_stream<'a>(&self, s: BoxStream<'a, Result<Bytes>>) -> Result<BoxStream<'a, Result<Bytes>>>
fn get_variant(&self) -> &CompressionTypeVariant
const fn is_compressed(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(t: CompressionTypeVariant) -> Self
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

**via `datafusion_common::file_options::file_type::GetExt`**

```rust
fn get_ext(&self) -> String
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_compression_type.FileCompressionType.md).


Readable file compression type

---

## FileTypeExt

`trait` · `datafusion_datasource::file_compression_type::FileTypeExt`

Also reachable as `datafusion::datasource::file_format::file_compression_type::FileTypeExt`

```rust
trait FileTypeExt
```

**Methods** (1)

```rust
fn get_ext_with_compression(&self, c: FileCompressionType) -> Result<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_compression_type.FileTypeExt.md).


Trait for extending the functionality of the `FileType` enum.

---
