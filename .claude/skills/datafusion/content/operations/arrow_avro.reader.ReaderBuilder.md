# `arrow_avro::reader::ReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.ReaderBuilder.json).

<a id="op-02775151633831d690c69c0f"></a>
## ReaderBuilder

`struct` · `arrow_avro::reader::ReaderBuilder` · arrow-avro 59.3.0

```rust
struct ReaderBuilder
```

Source: `src/reader/mod.rs:969`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A builder that configures and constructs Avro readers and decoders.

`ReaderBuilder` is the primary entry point for this module. It supports:

* OCF reading via `Self::build`, returning a `Reader` over any `BufRead`;
* streaming decoding via `Self::build_decoder`, returning a `Decoder`.

### Options

* **`batch_size`**: Max rows per `RecordBatch` (default: `1024`). See `Self::with_batch_size`.
* **`utf8_view`**: Use Arrow `StringViewArray` for string columns (default: `false`).
  See `Self::with_utf8_view`.
* **`strict_mode`**: Opt‑in to stricter union handling (default: `false`).
  See `Self::with_strict_mode`.
* **`reader_schema`**: Optional reader schema (projection / evolution) used when decoding
  values (default: `None`). See `Self::with_reader_schema`.
* **`projection`**: Optional projection of **top‑level record fields** by index (default: `None`).

  If set, the effective reader schema is **pruned** to include only the projected fields, in the
  specified order:

  * If a reader schema is provided, that schema is pruned.
  * Otherwise, a reader schema is derived from the writer schema and then pruned.
  * For streaming `Decoder` with multiple writer schemas and no reader schema, a projected reader
    schema is derived **per writer schema** in the `SchemaStore`.

  See `Self::with_projection`.
* **`writer_schema_store`**: Required for building a `Decoder` for single‑object or
  Confluent framing. Maps fingerprints to Avro schemas. See `Self::with_writer_schema_store`.
* **`active_fingerprint`**: Optional starting fingerprint for streaming decode when the
  first frame omits one (rare). See `Self::with_active_fingerprint`.

### Examples

Read an OCF file in batches of 4096 rows:

```no_run
use std::fs::File;
use std::io::BufReader;
use arrow_avro::reader::ReaderBuilder;

let file = File::open("data.avro")?;
let mut reader = ReaderBuilder::new()
    .with_batch_size(4096)
    .build(BufReader::new(file))?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Build a `Decoder` for Confluent messages:

```
use arrow_avro::schema::{AvroSchema, SchemaStore, Fingerprint, FingerprintAlgorithm};
use arrow_avro::reader::ReaderBuilder;

let mut store = SchemaStore::new_with_type(FingerprintAlgorithm::Id);
store.set(Fingerprint::Id(1234), AvroSchema::new(r#"{"type":"record","name":"E","fields":[]}"#.to_string()))?;

let decoder = ReaderBuilder::new()
    .with_writer_schema_store(store)
    .build_decoder()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

<a id="op-2e0aac0d92f5fb75e56eca88"></a>
## build

`function` · `arrow_avro::reader::ReaderBuilder::build` · arrow-avro 59.3.0

```rust
fn build<R: BufRead>(self, reader: R) -> Result<Reader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1290`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build a `Reader` (OCF) from this builder and a `BufRead`.

This reads and validates the OCF header, initializes an internal row decoder from
the discovered writer (and optional reader) schema, and prepares to iterate blocks,
decompressing if necessary.

<a id="op-a5acb452dada07a05ea6d14e"></a>
## build_decoder

`function` · `arrow_avro::reader::ReaderBuilder::build_decoder` · arrow-avro 59.3.0

```rust
fn build_decoder(self) -> Result<Decoder, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1313`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build a streaming `Decoder` from this builder.

# Requirements
* `SchemaStore` **must** be provided via `Self::with_writer_schema_store`.
* The store should contain **all** fingerprints that may appear on the stream.

# Errors
* Returns [`ArrowError::InvalidArgumentError`](../operations/arrow_schema.error.ArrowError.md#op-3a1a629c977ce0ac9c1d54a3) if the schema store is missing

<a id="op-ae65e56539ee0ac07d2115f8"></a>
## default

`function` · `arrow_avro::reader::ReaderBuilder::default` · arrow-avro 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [980, 1], "end": [993, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/reader/mod.rs:981`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe5383c7fc85c7dd9ebe4baf"></a>
## fmt

`function` · `arrow_avro::reader::ReaderBuilder::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 10], "end": [968, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1ca19424159c70d59dd4173"></a>
## new

`function` · `arrow_avro::reader::ReaderBuilder::new` · arrow-avro 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1006`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates a new `ReaderBuilder` with defaults:

* `batch_size = 1024`
* `strict_mode = false`
* `utf8_view = false`
* `tz = Tz::OffsetZero`
* `reader_schema = None`
* `projection = None`
* `writer_schema_store = None`
* `active_fingerprint = None`

<a id="op-28118828221d51d5f6c8c8ba"></a>
## use_utf8view

`function` · `arrow_avro::reader::ReaderBuilder::use_utf8view` · arrow-avro 59.3.0

```rust
fn use_utf8view(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1170`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns whether `StringViewArray` is enabled for string data.

<a id="op-902b2b67b8c3124efc777b1e"></a>
## with_active_fingerprint

`function` · `arrow_avro::reader::ReaderBuilder::with_active_fingerprint` · arrow-avro 59.3.0

```rust
fn with_active_fingerprint(self, fp: Fingerprint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1280`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the initial schema fingerprint for stream decoding.

This can be useful for streams that **do not include** a fingerprint before the first
record body (uncommon). If not set, the first observed fingerprint is used.

<a id="op-ba0a1e0a4f4d2236edc1ce5b"></a>
## with_batch_size

`function` · `arrow_avro::reader::ReaderBuilder::with_batch_size` · arrow-avro 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1154`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the **row‑based batch size**.

Each call to `Decoder::flush` or each iteration of `Reader` yields a batch with
*up to* this many rows. Larger batches can reduce overhead; smaller batches can
reduce peak memory usage and latency.

<a id="op-3054a874643620d304081d6d"></a>
## with_projection

`function` · `arrow_avro::reader::ReaderBuilder::with_projection` · arrow-avro 59.3.0

```rust
fn with_projection(self, projection: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1259`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets an explicit top-level field projection by index.

The provided `projection` is a list of indices into the **top-level record** fields.
The output schema will contain only these fields, in the specified order.

Internally, this is implemented by pruning the effective Avro *reader schema*:

* If a reader schema is provided via `Self::with_reader_schema`, that schema is pruned.
* Otherwise, a reader schema is derived from the writer schema and then pruned.
* For streaming `Decoder` with multiple writer schemas and no reader schema, a projected
  reader schema is derived **per writer schema** in the `SchemaStore`.

# Example

Read only specific columns from an Avro OCF file:

```
use std::io::Cursor;
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, StringArray, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroWriter;
use arrow_avro::reader::ReaderBuilder;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Original schema has three fields: id, name, value
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("name", DataType::Utf8, false),
    Field::new("value", DataType::Float64, false),
]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![
        Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef,
        Arc::new(StringArray::from(vec!["a", "b", "c"])) as ArrayRef,
        Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0])) as ArrayRef,
    ],
)?;

// Write Avro OCF
let mut writer = AvroWriter::new(Vec::new(), schema)?;
writer.write(&batch)?;
writer.finish()?;
let bytes = writer.into_inner();

// Read only fields at indices 2 and 0 (value, id) — in that order
let mut reader = ReaderBuilder::new()
    .with_projection(vec![2, 0])
    .build(Cursor::new(bytes))?;

let out = reader.next().unwrap()?;
assert_eq!(out.num_columns(), 2);
assert_eq!(out.schema().field(0).name(), "value");
assert_eq!(out.schema().field(1).name(), "id");
# Ok(()) }
```

<a id="op-3845c41d7c0832f8c5314c5e"></a>
## with_reader_schema

`function` · `arrow_avro::reader::ReaderBuilder::with_reader_schema` · arrow-avro 59.3.0

```rust
fn with_reader_schema(self, schema: AvroSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1197`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the **reader schema** used during decoding.

If not provided, the writer schema from the OCF header (for `Reader`) or the
schema looked up from the fingerprint (for `Decoder`) is used directly.

A reader schema can be used for **schema evolution** or **projection**.

<a id="op-9fdacfb14ae892049be5da4e"></a>
## with_strict_mode

`function` · `arrow_avro::reader::ReaderBuilder::with_strict_mode` · arrow-avro 59.3.0

```rust
fn with_strict_mode(self, strict_mode: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1178`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Enable stricter behavior for certain Avro unions (e.g., `[T, "null"]`).

When `true`, ambiguous or lossy unions that would otherwise be coerced may instead
produce a descriptive error. Use this to catch schema issues early during ingestion.

<a id="op-da26306fe6cde3d9184f6235"></a>
## with_tz

`function` · `arrow_avro::reader::ReaderBuilder::with_tz` · arrow-avro 59.3.0

```rust
fn with_tz(self, tz: Tz) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1186`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the timezone representation for Avro timestamp fields.

The default is `Tz::OffsetZero`, meaning the "+00:00" time zone ID.

<a id="op-fb4694a950ad128e1f73adf3"></a>
## with_utf8_view

`function` · `arrow_avro::reader::ReaderBuilder::with_utf8_view` · arrow-avro 59.3.0

```rust
fn with_utf8_view(self, utf8_view: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1164`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Choose Arrow's `StringViewArray` for UTF‑8 string data.

When enabled, textual Avro fields are loaded into Arrow’s **StringViewArray**
instead of the standard `StringArray`. This can improve performance for workloads
with many short strings by reducing allocations.

<a id="op-26ae6b26e8d8175d6c8662b5"></a>
## with_writer_schema_store

`function` · `arrow_avro::reader::ReaderBuilder::with_writer_schema_store` · arrow-avro 59.3.0

```rust
fn with_writer_schema_store(self, store: SchemaStore) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1322, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1271`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the `SchemaStore` used to resolve writer schemas by fingerprint.

This is required when building a `Decoder` for **single‑object encoding** or the
**Confluent** wire format. The store maps a fingerprint (Rabin / MD5 / SHA‑256 /
ID) to a full Avro schema.

Defaults to `None`.
