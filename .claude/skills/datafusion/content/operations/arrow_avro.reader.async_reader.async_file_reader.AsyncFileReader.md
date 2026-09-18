# `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.json).

<a id="op-b37e46224b914587014a7456"></a>
## AsyncFileReader

`trait` · `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader` · arrow-avro 59.3.0

```rust
trait AsyncFileReader: Send
```

Source: `src/reader/async_reader/async_file_reader.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

The asynchronous interface used by [`super::AsyncAvroFileReader`](../operations/arrow_avro.reader.async_reader.AsyncAvroFileReader.md#op-1556b0e38a5fc44a0cc6248c) to read avro files

Notes:

1. There is a default implementation for types that implement [`AsyncRead`]
   and [`AsyncSeek`], for example [`tokio::fs::File`].

2. Implementations for remote storage, such as the [`object_store`] crate,
   can implement this interface directly, typically by pairing a store
   handle with an object path and delegating [`Self::get_bytes`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-ff3557fbfe855f49413ddd9d) and
   [`Self::get_byte_ranges`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-282637cd7c3cc0eab61ebd91) to ranged reads. [`super::SpawnedReader`](../operations/arrow_avro.reader.async_reader.spawn.SpawnedReader.md#op-afdf2c39ce6cfca5ab90e59a) can
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

Unresolved upstream links (retained, not inferred): ``AsyncSeek``, ``AsyncRead``.

<a id="op-282637cd7c3cc0eab61ebd91"></a>
## get_byte_ranges

`function` · `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader::get_byte_ranges` · arrow-avro 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>>
```

Source: `src/reader/async_reader/async_file_reader.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Retrieve multiple byte ranges. The default implementation will call `get_bytes` sequentially

<a id="op-ff3557fbfe855f49413ddd9d"></a>
## get_bytes

`function` · `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader::get_bytes` · arrow-avro 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

Source: `src/reader/async_reader/async_file_reader.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Retrieve the bytes in `range`
