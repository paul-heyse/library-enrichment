# `arrow_avro::writer::Writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.Writer.json).

<a id="op-2a9e145d555e09d95bf56622"></a>
## Writer

`struct` · `arrow_avro::writer::Writer` · arrow-avro 59.3.0

```rust
struct Writer<W: Write, F: AvroFormat>
```

Source: `src/writer/mod.rs:564`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Generic Avro writer.

This type is generic over the output Write sink (`W`) and the Avro format (`F`).
You’ll usually use the concrete aliases:

* **[`AvroWriter`](../operations/arrow_avro.writer.AvroWriter.md#op-109f7d6c1f115d2ec6574736)** for **OCF** (self‑describing container file)
* **[`AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550)** for **SOE** Avro streams

<a id="op-d0dcffbbc54d94859fd39823"></a>
## finish

`function` · `arrow_avro::writer::Writer::finish` · arrow-avro 59.3.0

```rust
fn finish(&mut self) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [717, 1], "end": [775, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:742`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Flush remaining buffered data and (for OCF) ensure the header is present.

<a id="op-1d1cf4ee9b1cb1c9ad6c5aa0"></a>
## fmt

`function` · `arrow_avro::writer::Writer::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 10], "end": [563, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:563`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec086a71e3b58a04baa594b9"></a>
## into_inner

`function` · `arrow_avro::writer::Writer::into_inner` · arrow-avro 59.3.0

```rust
fn into_inner(self) -> W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [717, 1], "end": [775, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:749`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Consume the writer, returning the underlying output object.

<a id="op-127efa1a04756952a3ce615a"></a>
## new

`function` · `arrow_avro::writer::Writer::new` · arrow-avro 59.3.0

```rust
fn new(writer: W, schema: Schema) -> Result<Self, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "crate::writer::format::AvroOcfFormat"}}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 1], "end": [682, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:674`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Convenience constructor – same as [`WriterBuilder::build`](../operations/arrow_avro.writer.WriterBuilder.md#op-51226c47f6c1e887614eea9d) with `AvroOcfFormat`.

### Example

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroWriter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef],
)?;

let buf: Vec<u8> = Vec::new();
let mut w = AvroWriter::new(buf, schema)?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();
assert!(!bytes.is_empty());
# Ok(()) }
```

<a id="op-8b9936a67a90ac21fd1378da"></a>
## new

`function` · `arrow_avro::writer::Writer::new` · arrow-avro 59.3.0

```rust
fn new(writer: W, schema: Schema) -> Result<Self, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "crate::writer::format::AvroSoeFormat"}}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [715, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:712`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Convenience constructor to create a new [`AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550).

The resulting stream contains **Single Object Encodings** (no OCF header/sync).

### Example

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroStreamWriter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("x", DataType::Int64, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int64Array::from(vec![10, 20])) as ArrayRef],
)?;

let sink: Vec<u8> = Vec::new();
let mut w = AvroStreamWriter::new(sink, schema)?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();
assert!(!bytes.is_empty());
# Ok(()) }
```

<a id="op-dcddb3f434a813474a6d325f"></a>
## sync_marker

`function` · `arrow_avro::writer::Writer::sync_marker` · arrow-avro 59.3.0

```rust
fn sync_marker(&self) -> Option<&[u8; 16]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "crate::writer::format::AvroOcfFormat"}}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 1], "end": [682, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:679`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Return a reference to the 16‑byte sync marker generated for this file.

<a id="op-c113ce42dae033d5377febf6"></a>
## write

`function` · `arrow_avro::writer::Writer::write` · arrow-avro 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [717, 1], "end": [775, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:719`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Serialize one [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to the output.

<a id="op-bb9e79d83228d07926434e32"></a>
## write_batches

`function` · `arrow_avro::writer::Writer::write_batches` · arrow-avro 59.3.0

```rust
fn write_batches(&mut self, batches: &[&RecordBatch]) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "arrow_avro::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [717, 1], "end": [775, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:734`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A convenience method to write a slice of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

This is equivalent to calling `write` for each batch in the slice.
