# `arrow_json::writer::Writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.Writer.json).

<a id="op-e929064a2c0f20a280aeb9ef"></a>
## Writer

`struct` · `arrow_json::writer::Writer` · arrow-json 59.3.0

```rust
struct Writer<W, F> where W: Write, F: JsonFormat
```

Source: `src/writer/mod.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A JSON writer which serializes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to a stream of
`u8` encoded JSON objects.

See the module level documentation for detailed usage and examples.
The specific format of the stream is controlled by the [`JsonFormat`](../operations/arrow_json.writer.JsonFormat.md#op-3a011970d5da2ba6a8b56ed1)
type parameter.

By default the writer will skip writing keys with null values for
backward compatibility. See [`WriterBuilder`](../operations/arrow_json.writer.WriterBuilder.md#op-eab011f6b5e700c1657bc7e0) on how to customize
this behaviour when creating a new writer.

<a id="op-0499ce8daad1166cde1b5889"></a>
## close

`function` · `arrow_json::writer::Writer::close` · arrow-json 59.3.0

```rust
fn close(self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [465, 1], "end": [477, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer/mod.rs:474`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada81fd5db88e7c89f56c96a"></a>
## finish

`function` · `arrow_json::writer::Writer::finish` · arrow-json 59.3.0

```rust
fn finish(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:433`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Finishes the output stream. This function must be called after
all record batches have been produced. (e.g. producing the final `']'` if writing
arrays.

<a id="op-41acba9e1d79aee771c6da05"></a>
## fmt

`function` · `arrow_json::writer::Writer::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [338, 10], "end": [338, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8bd0fc07899dcf08620032b"></a>
## get_mut

`function` · `arrow_json::writer::Writer::get_mut` · arrow-json 59.3.0

```rust
fn get_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Gets a mutable reference to the underlying writer.

Writing to the underlying writer must be done with care
to avoid corrupting the output JSON.

<a id="op-39672a059f75671f4805cef9"></a>
## get_ref

`function` · `arrow_json::writer::Writer::get_ref` · arrow-json 59.3.0

```rust
fn get_ref(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:447`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Gets a reference to the underlying writer.

<a id="op-e4a4eb0a16592e529a90c3f9"></a>
## into_inner

`function` · `arrow_json::writer::Writer::into_inner` · arrow-json 59.3.0

```rust
fn into_inner(self) -> W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Unwraps this `Writer<W>`, returning the underlying writer

<a id="op-9c5827695b0fe86f6425ac33"></a>
## new

`function` · `arrow_json::writer::Writer::new` · arrow-json 59.3.0

```rust
fn new(writer: W) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:366`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Construct a new writer

<a id="op-0486a5e32a2c79c185d00a71"></a>
## write

`function` · `arrow_json::writer::Writer::write` · arrow-json 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [465, 1], "end": [477, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer/mod.rs:470`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c55bc528f7a6efb37a9ddcd"></a>
## write

`function` · `arrow_json::writer::Writer::write` · arrow-json 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Serialize `batch` to JSON output

<a id="op-17d5c2dd166c89a5cac0117b"></a>
## write_batches

`function` · `arrow_json::writer::Writer::write_batches` · arrow-json 59.3.0

```rust
fn write_batches(&mut self, batches: &[&RecordBatch]) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_json::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "generic_params": [], "type": {"generic": "W"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [360, 1], "end": [463, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:423`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Serialize `batches` to JSON output
