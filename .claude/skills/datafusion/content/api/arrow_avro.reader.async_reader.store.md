# `arrow_avro::reader::async_reader::store`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.reader.async_reader.store.json`](../model/arrow_avro.reader.async_reader.store.json)

## AvroObjectReader

`struct` · `arrow_avro::reader::async_reader::store::AvroObjectReader`

> **Deprecated** — since 59.2.0: Implement `AsyncFileReader` directly instead; see the example on the `AsyncFileReader` trait documentation and `arrow-avro/examples/object_store.rs`. Use `SpawnedReader` to perform I/O on a dedicated runtime. See also https://github.com/apache/arrow-rs/issues/10308

Also reachable as `arrow_avro::reader::AvroObjectReader`, `arrow_avro::reader::async_reader::AvroObjectReader`

```rust
struct AvroObjectReader
```

**Implements**: `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
fn with_runtime(self, handle: Handle) -> Self
```

**via `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`**

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>> where Self: Send
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

An implementation of an AsyncFileReader using the [`ObjectStore`] API.

---
