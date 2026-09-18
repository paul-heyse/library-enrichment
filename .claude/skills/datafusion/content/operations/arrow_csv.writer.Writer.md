# `arrow_csv::writer::Writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.writer.Writer.json).

<a id="op-9112f918255e66827bb62542"></a>
## Writer

`struct` · `arrow_csv::writer::Writer` · arrow-csv 59.3.0

```rust
struct Writer<W: Write>
```

Source: `src/writer.rs:205`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

A CSV writer

See the [module documentation](crate::writer) for examples.

<a id="op-55d84af2d90e3acdecde9555"></a>
## close

`function` · `arrow_csv::writer::Writer::close` · arrow-csv 59.3.0

```rust
fn close(self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 1], "end": [348, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30d594c4b7e5f1679f4cf961"></a>
## fmt

`function` · `arrow_csv::writer::Writer::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 10], "end": [204, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e104ed3ddaba9f987efd907"></a>
## into_inner

`function` · `arrow_csv::writer::Writer::into_inner` · arrow-csv 59.3.0

```rust
fn into_inner(self) -> W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [338, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Unwraps this `Writer<W>`, returning the underlying writer.

<a id="op-b9559c6ba5ddb68699071991"></a>
## new

`function` · `arrow_csv::writer::Writer::new` · arrow-csv 59.3.0

```rust
fn new(writer: W) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [338, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new CsvWriter from a writable object, with default options

See [`WriterBuilder`](../operations/arrow_csv.writer.WriterBuilder.md#op-44e2845dc1cd29d45ec5dba4) for configure options, and the [module
documentation](crate::writer) for examples.

<a id="op-ca949f627ba5c612478c962a"></a>
## write

`function` · `arrow_csv::writer::Writer::write` · arrow-csv 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [338, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Write a RecordBatch to the underlying writer

<a id="op-d46abf0b62fa3d381c5053b9"></a>
## write

`function` · `arrow_csv::writer::Writer::write` · arrow-csv 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_csv::writer::Writer", "path": "Writer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 1], "end": [348, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:341`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
