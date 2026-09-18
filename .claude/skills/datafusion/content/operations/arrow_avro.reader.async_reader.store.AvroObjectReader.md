# `arrow_avro::reader::async_reader::store::AvroObjectReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.store.AvroObjectReader.json).

<a id="op-98aa93fe79d1ec5a6f9b6359"></a>
## AvroObjectReader

`struct` · `arrow_avro::reader::async_reader::store::AvroObjectReader` · arrow-avro 59.3.0

```rust
struct AvroObjectReader
```

Source: `src/reader/async_reader/store.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

An implementation of an AsyncFileReader using the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) API.

<a id="op-367416b79d7323d2a3894641"></a>
## clone

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> AvroObjectReader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/reader/async_reader/store.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/reader/async_reader/store.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e97c4e16578e895cf81847d"></a>
## fmt

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/reader/async_reader/store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/async_reader/store.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8318270880d20a7a92ba8fb"></a>
## get_byte_ranges

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::get_byte_ranges` · arrow-avro 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>> where Self: Send
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [118, 2], "filename": "src/reader/async_reader/store.rs"}, "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader"}`

Source: `src/reader/async_reader/store.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d886de9b59c5746e49f74a64"></a>
## get_bytes

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::get_bytes` · arrow-avro 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [118, 2], "filename": "src/reader/async_reader/store.rs"}, "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader"}`

Source: `src/reader/async_reader/store.rs:105`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72c59fda65e23a4c297074db"></a>
## new

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::new` · arrow-avro 59.3.0

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [101, 2], "filename": "src/reader/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/store.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates a new [`Self`](../operations/arrow_avro.reader.async_reader.store.AvroObjectReader.md#op-98aa93fe79d1ec5a6f9b6359) from a store implementation and file location.

<a id="op-4b688bf0136bdeedb55703e4"></a>
## with_runtime

`function` · `arrow_avro::reader::async_reader::store::AvroObjectReader::with_runtime` · arrow-avro 59.3.0

```rust
fn with_runtime(self, handle: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::async_reader::store::AvroObjectReader", "path": "AvroObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [101, 2], "filename": "src/reader/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/store.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Perform IO on the provided tokio runtime

Tokio is a cooperative scheduler, and relies on tasks yielding in a timely manner
to service IO. Therefore, running IO and CPU-bound tasks, such as avro decoding,
on the same tokio runtime can lead to degraded throughput, dropped connections and
other issues. For more information see [here].

[here]: https://www.influxdata.com/blog/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/
