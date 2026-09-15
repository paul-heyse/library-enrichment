# `arrow_avro::reader::async_reader::builder`

Crate `arrow-avro` · 2 public items · structured records in [`model/arrow_avro.reader.async_reader.builder.json`](../model/arrow_avro.reader.async_reader.builder.json)

## read_header_info

`function` · `arrow_avro::reader::async_reader::builder::read_header_info`

Also reachable as `arrow_avro::reader::async_reader::read_header_info`

```rust
async fn read_header_info<R>(reader: &mut R, file_size: u64, header_size_hint: Option<u64>) -> Result<reader::header::HeaderInfo, errors::AvroError> where R: AsyncFileReader
```

Reads the Avro file header (magic, metadata, sync marker) asynchronously from `reader`.

On success, returns the parsed [`HeaderInfo`] containing the header and its length in bytes.

---

## ReaderBuilder

`struct` · `arrow_avro::reader::async_reader::builder::ReaderBuilder`

Also reachable as `arrow_avro::reader::async_reader::ReaderBuilder`

```rust
struct ReaderBuilder<R>
```

**Methods** (9)

```rust
fn build_with_header(self, header_info: HeaderInfo) -> Result<AsyncAvroFileReader<R>, AvroError>
async fn try_build(self) -> Result<AsyncAvroFileReader<R>, AvroError>
fn with_header_size_hint(self, hint: u64) -> Self
fn with_projection(self, projection: Vec<usize>) -> Self
fn with_range(self, range: Range<u64>) -> Self
fn with_reader_schema(self, reader_schema: AvroSchema) -> Self
fn with_strict_mode(self, strict_mode: bool) -> Self
fn with_tz(self, tz: Tz) -> Self
fn with_utf8_view(self, utf8_view: bool) -> Self
```

Builder for an asynchronous Avro file reader.

---
