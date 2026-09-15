# `arrow_json::writer`

Crate `arrow-json` · 7 public items · structured records in [`model/arrow_json.writer.json`](../model/arrow_json.writer.json)

## JsonArray

`struct` · `arrow_json::writer::JsonArray`

```rust
struct JsonArray
```

**Implements**: `arrow_json::writer::JsonFormat`

**Derives**: Debug, Default

**via `arrow_json::writer::JsonFormat`**

```rust
fn end_stream<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
fn start_row<W: Write>(&self, writer: &mut W, is_first_row: bool) -> Result<(), ArrowError>
fn start_stream<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
```

Produces JSON output as a single JSON array.

For example:

```json
[{"foo":1},{"bar":1}]
```

---

## LineDelimited

`struct` · `arrow_json::writer::LineDelimited`

```rust
struct LineDelimited
```

**Implements**: `arrow_json::writer::JsonFormat`

**Derives**: Debug, Default

**via `arrow_json::writer::JsonFormat`**

```rust
fn end_row<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
```

Produces JSON output with one record per line.

For example:

```json
{"foo":1}
{"bar":1}

```

---

## Writer

`struct` · `arrow_json::writer::Writer`

Also reachable as `arrow::json::Writer`, `arrow_json::Writer`

```rust
struct Writer<W, F> where W: Write, F: JsonFormat
```

**Implements**: `arrow_array::record_batch::RecordBatchWriter`

**Derives**: Debug

**Methods** (7)

```rust
fn finish(&mut self) -> Result<(), ArrowError>
fn get_mut(&mut self) -> &mut W
fn get_ref(&self) -> &W
fn into_inner(self) -> W
fn new(writer: W) -> Self
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
fn write_batches(&mut self, batches: &[&RecordBatch]) -> Result<(), ArrowError>
```

**via `arrow_array::record_batch::RecordBatchWriter`**

```rust
fn close(self) -> Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

A JSON writer which serializes [`RecordBatch`]es to a stream of
`u8` encoded JSON objects.

See the module level documentation for detailed usage and examples.
The specific format of the stream is controlled by the [`JsonFormat`]
type parameter.

By default the writer will skip writing keys with null values for
backward compatibility. See [`WriterBuilder`] on how to customize
this behaviour when creating a new writer.

---

## WriterBuilder

`struct` · `arrow_json::writer::WriterBuilder`

Also reachable as `arrow::json::WriterBuilder`, `arrow_json::WriterBuilder`

```rust
struct WriterBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (12)

```rust
fn build<W, F>(self, writer: W) -> Writer<W, F> where W: Write, F: JsonFormat
fn explicit_nulls(&self) -> bool
fn new() -> Self
fn struct_mode(&self) -> StructMode
fn with_date_format(self, format: String) -> Self
fn with_datetime_format(self, format: String) -> Self
fn with_encoder_factory(self, factory: Arc<dyn EncoderFactory>) -> Self
fn with_explicit_nulls(self, explicit_nulls: bool) -> Self
fn with_struct_mode(self, struct_mode: StructMode) -> Self
fn with_time_format(self, format: String) -> Self
fn with_timestamp_format(self, format: String) -> Self
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

JSON writer builder.

---

## JsonFormat

`trait` · `arrow_json::writer::JsonFormat`

```rust
trait JsonFormat: Debug + Default
```

**Implementors** (2)

- `arrow_json::writer::JsonArray`
- `arrow_json::writer::LineDelimited`

**Methods** (4)

```rust
fn end_row<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
fn end_stream<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
fn start_row<W: Write>(&self, _writer: &mut W, _is_first_row: bool) -> Result<(), ArrowError>
fn start_stream<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
```

This trait defines how to format a sequence of JSON objects to a
byte stream.

---

## ArrayWriter

`type_alias` · `arrow_json::writer::ArrayWriter`

Also reachable as `arrow::json::ArrayWriter`, `arrow_json::ArrayWriter`

```rust
type ArrayWriter<W> = Writer<W, JsonArray>
```

A JSON writer which serializes [`RecordBatch`]es to JSON arrays.

---

## LineDelimitedWriter

`type_alias` · `arrow_json::writer::LineDelimitedWriter`

Also reachable as `arrow::json::LineDelimitedWriter`, `arrow_json::LineDelimitedWriter`

```rust
type LineDelimitedWriter<W> = Writer<W, LineDelimited>
```

A JSON writer which serializes [`RecordBatch`]es to newline delimited JSON objects.

---
