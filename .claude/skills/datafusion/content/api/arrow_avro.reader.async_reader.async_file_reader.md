# `arrow_avro::reader::async_reader::async_file_reader`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.reader.async_reader.async_file_reader.json`](../model/arrow_avro.reader.async_reader.async_file_reader.json)

## AsyncFileReader

`trait` · `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`

Also reachable as `arrow_avro::reader::AsyncFileReader`, `arrow_avro::reader::async_reader::AsyncFileReader`

```rust
trait AsyncFileReader: Send
```

**Implementors** (3)

- `alloc::boxed::Box`
- `arrow_avro::reader::async_reader::spawn::SpawnedReader`
- `arrow_avro::reader::async_reader::store::AvroObjectReader`

**Methods** (2)

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>>
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md).


The asynchronous interface used by [`super::AsyncAvroFileReader`] to read avro files

Notes:

1. There is a default implementation for types that implement [`AsyncRead`]
   and [`AsyncSeek`], for example [`tokio::fs::File`].

2. Implementations for remote storage, such as the [`object_store`] crate,
   can implement this interface directly, typically by pairing a store
   handle with an object path and delegating [`Self::get_bytes`] and
   [`Self::get_byte_ranges`] to ranged reads. [`super::SpawnedReader`] can
   wrap such a reader to perform its I/O on a dedicated tokio runtime.

[`object_store`]: https://crates.io/crates/object_store

# Example: implementing `AsyncFileReader` for the `object_store` crate

```no_run
# use std::ops::Range;
# use std::sync::Arc;
use arrow_avro::errors::AvroError;
use arrow_avro::reader::AsyncFileReader;
use bytes::Bytes;
use futures::FutureExt;
use futures::future::BoxFuture;
use object_store::path::Path;
use object_store::{ObjectStore, ObjectStoreExt};

#[derive(Clone, Debug)]
struct ObjectStoreReader {
    store: Arc<dyn ObjectStore>,
    path: Path,
}

impl AsyncFileReader for ObjectStoreReader {
    fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>> {
        async move {
            self.store
                .get_range(&self.path, range)
                .await
                .map_err(|e| AvroError::General(e.to_string()))
        }
        .boxed()
    }

    fn get_byte_ranges(
        &mut self,
        ranges: Vec<Range<u64>>,
    ) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>> {
        async move {
            self.store
                .get_ranges(&self.path, &ranges)
                .await
                .map_err(|e| AvroError::General(e.to_string()))
        }
        .boxed()
    }
}
```

[`tokio::fs::File`]: https://docs.rs/tokio/latest/tokio/fs/struct.File.html

---
