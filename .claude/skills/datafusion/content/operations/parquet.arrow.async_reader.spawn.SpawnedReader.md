# `parquet::arrow::async_reader::spawn::SpawnedReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.spawn.SpawnedReader.json).

<a id="op-e0eacd10964c1557d2522d82"></a>
## SpawnedReader

`struct` · `parquet::arrow::async_reader::spawn::SpawnedReader` · parquet 59.3.0

```rust
struct SpawnedReader<R>
```

Source: `src/arrow/async_reader/spawn.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An [`AsyncFileReader`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-69c2992a81570b27e07fe176) that performs I/O on a separate tokio runtime.

Tokio is a cooperative scheduler, and relies on tasks yielding in a timely
manner to service IO. Therefore, running IO and CPU-bound tasks, such as
parquet decoding, on the same tokio runtime can lead to degraded
throughput, dropped connections and other issues. For more information see
[here].

This wrapper spawns each operation of the inner reader onto the provided
runtime [`Handle`], so that the runtime driving the parquet decoding does
not also drive the I/O.

Note that [`Self::get_metadata`](../operations/parquet.arrow.async_reader.spawn.SpawnedReader.md#op-3494fc0c3acc1561ccca3895) spawns the entire metadata load, so the
footer is also decoded on the provided runtime.

The inner reader must be [`Clone`] (typically an `Arc`'d handle to some
shared resource) as each spawned task requires a `'static` copy of it.

[here]: https://www.influxdata.com/blog/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/

Unresolved upstream links (retained, not inferred): ``Handle``, ``Clone``.

<a id="op-c4da68d6f53d44c3d61b598f"></a>
## clone

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::clone` · parquet 59.3.0

```rust
fn clone(&self) -> SpawnedReader<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/async_reader/spawn.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cba3b58d20039deb17fe1382"></a>
## fmt

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/async_reader/spawn.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c7be6742edc5f56a0b0dd6"></a>
## get_byte_ranges

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::get_byte_ranges` · parquet 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [90, 1], "end": [117, 2], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/spawn.rs:99`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b457ef294922fb5482f0e144"></a>
## get_bytes

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [90, 1], "end": [117, 2], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/spawn.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3494fc0c3acc1561ccca3895"></a>
## get_metadata

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::get_metadata` · parquet 59.3.0

```rust
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [90, 1], "end": [117, 2], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/spawn.rs:107`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dda3da48dd96c3f2b607277"></a>
## into_inner

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [67, 2], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/spawn.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the inner reader

<a id="op-5e3bfc78973d3d569a0da79b"></a>
## new

`function` · `parquet::arrow::async_reader::spawn::SpawnedReader::new` · parquet 59.3.0

```rust
fn new(inner: R, handle: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::spawn::SpawnedReader", "path": "SpawnedReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [67, 2], "filename": "src/arrow/async_reader/spawn.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/spawn.rs:59`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new [`SpawnedReader`](../operations/parquet.arrow.async_reader.spawn.SpawnedReader.md#op-e0eacd10964c1557d2522d82) that performs the I/O of `inner` on `handle`
