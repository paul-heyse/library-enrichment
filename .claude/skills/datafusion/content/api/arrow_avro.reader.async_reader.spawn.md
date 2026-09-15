# `arrow_avro::reader::async_reader::spawn`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.reader.async_reader.spawn.json`](../model/arrow_avro.reader.async_reader.spawn.json)

## SpawnedReader

`struct` · `arrow_avro::reader::async_reader::spawn::SpawnedReader`

Also reachable as `arrow_avro::reader::SpawnedReader`, `arrow_avro::reader::async_reader::SpawnedReader`

```rust
struct SpawnedReader<R>
```

**Implements**: `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn into_inner(self) -> R
fn new(inner: R, handle: Handle) -> Self
```

**via `arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader`**

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>>
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

An [`AsyncFileReader`] that performs I/O on a separate tokio runtime.

Tokio is a cooperative scheduler, and relies on tasks yielding in a timely
manner to service IO. Therefore, running IO and CPU-bound tasks, such as
avro decoding, on the same tokio runtime can lead to degraded throughput,
dropped connections and other issues. For more information see [here].

This wrapper spawns each operation of the inner reader onto the provided
runtime [`Handle`], so that the runtime driving the avro decoding does not
also drive the I/O.

The inner reader must be [`Clone`] (typically an `Arc`'d handle to some
shared resource) as each spawned task requires a `'static` copy of it.

[here]: https://www.influxdata.com/blog/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/

---
