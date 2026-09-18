# `datafusion_datasource::write`

Crate `datafusion-datasource` · 4 public items · structured records in [`model/datafusion_datasource.write.json`](../model/datafusion_datasource.write.json)

## get_writer_schema

`function` · `datafusion_datasource::write::get_writer_schema`

Also reachable as `datafusion::datasource::file_format::write::get_writer_schema`

```rust
fn get_writer_schema(config: &file_sink_config::FileSinkConfig) -> std::sync::Arc<arrow::datatypes::Schema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.get_writer_schema.md).


Converts table schema to writer schema, which may differ in the case
of hive style partitioning where some columns are removed from the
underlying files.

---

## ObjectWriterBuilder

`struct` · `datafusion_datasource::write::ObjectWriterBuilder`

Also reachable as `datafusion::datasource::file_format::write::ObjectWriterBuilder`

```rust
struct ObjectWriterBuilder
```

**Derives**: Debug

**Methods** (8)

```rust
fn build(self) -> Result<Box<dyn AsyncWrite + Send + Unpin>>
fn get_buffer_size(&self) -> Option<usize>
fn get_compression_level(&self) -> Option<u32>
fn new(file_compression_type: FileCompressionType, location: &Path, object_store: Arc<dyn ObjectStore>) -> Self
fn set_buffer_size(&mut self, buffer_size: Option<usize>)
fn set_compression_level(&mut self, compression_level: Option<u32>)
fn with_buffer_size(self, buffer_size: Option<usize>) -> Self
fn with_compression_level(self, compression_level: Option<u32>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.ObjectWriterBuilder.md).


A builder for an [`AsyncWrite`] that writes to an object store location.

This can be used to specify file compression on the writer. The writer
will have a default buffer size unless altered. The specific default size
is chosen by [`BufWriter::new`].

We drop the `AbortableWrite` struct and the writer will not try to cleanup on failure.
Users can configure automatic cleanup with their cloud provider.

---

## SharedBuffer

`struct` · `datafusion_datasource::write::SharedBuffer`

Also reachable as `datafusion::datasource::file_format::write::SharedBuffer`

```rust
struct SharedBuffer
```

**Fields**: `buffer`

**Implements**: `core::io::write::Write`

**Derives**: Clone

**Methods** (1)

```rust
fn new(capacity: usize) -> Self
```

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> std::io::Result<()>
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.SharedBuffer.md).


A buffer with interior mutability shared by the SerializedFileWriter and
ObjectStore writer

---

## BatchSerializer

`trait` · `datafusion_datasource::write::BatchSerializer`

Also reachable as `datafusion::datasource::file_format::write::BatchSerializer`

```rust
trait BatchSerializer: Sync + Send
```

**Implementors** (2)

- `datafusion_datasource_csv::file_format::CsvSerializer`
- `datafusion_datasource_json::file_format::JsonSerializer`

**Methods** (1)

```rust
fn serialize(&self, batch: RecordBatch, initial: bool) -> Result<Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.BatchSerializer.md).


A trait that defines the methods required for a RecordBatch serializer.

---
