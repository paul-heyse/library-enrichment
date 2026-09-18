# `arrow_avro::reader::async_reader::spawn::SpawnedReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.spawn.SpawnedReader.json).

<a id="op-afdf2c39ce6cfca5ab90e59a"></a>
## SpawnedReader

`struct` · `arrow_avro::reader::async_reader::spawn::SpawnedReader` · arrow-avro 59.3.0

```rust
struct SpawnedReader<R>
```

Source: `src/reader/async_reader/spawn.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

An [`AsyncFileReader`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-b37e46224b914587014a7456) that performs I/O on a separate tokio runtime.

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

Unresolved upstream links (retained, not inferred): ``Clone``, ``Handle``.

<a id="op-ec67ba63bb9785573d9fd143"></a>
## clone

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> SpawnedReader<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/reader/async_reader/spawn.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/reader/async_reader/spawn.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8874027a58f8290959bfc2"></a>
## fmt

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 22], "filename": "src/reader/async_reader/spawn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/async_reader/spawn.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85f99ae21489649d398fa278"></a>
## get_byte_ranges

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::get_byte_ranges` · arrow-avro 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>, AvroError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [83, 1], "end": [102, 2], "filename": "src/reader/async_reader/spawn.rs"}, "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader"}`

Source: `src/reader/async_reader/spawn.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6754721270d33ac112f482df"></a>
## get_bytes

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::get_bytes` · arrow-avro 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, AvroError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [83, 1], "end": [102, 2], "filename": "src/reader/async_reader/spawn.rs"}, "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader"}`

Source: `src/reader/async_reader/spawn.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c85b002ea3100297dda6cfdd"></a>
## into_inner

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::into_inner` · arrow-avro 59.3.0

```rust
fn into_inner(self) -> R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/reader/async_reader/spawn.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/spawn.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the inner reader

<a id="op-871ba2d8559663cda520b3ef"></a>
## new

`function` · `arrow_avro::reader::async_reader::spawn::SpawnedReader::new` · arrow-avro 59.3.0

```rust
fn new(inner: R, handle: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/reader/async_reader/spawn.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/spawn.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates a new [`SpawnedReader`](../operations/arrow_avro.reader.async_reader.spawn.SpawnedReader.md#op-afdf2c39ce6cfca5ab90e59a) that performs the I/O of `inner` on `handle`
