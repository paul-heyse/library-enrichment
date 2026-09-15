# `arrow_json::reader`

Crate `arrow-json` · 4 public items · structured records in [`model/arrow_json.reader.json`](../model/arrow_json.reader.json)

## Decoder

`struct` · `arrow_json::reader::Decoder`

```rust
struct Decoder
```

**Derives**: Debug

**Methods** (6)

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
fn has_partial_record(&self) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn serialize<S: Serialize>(&mut self, rows: &[S]) -> Result<(), ArrowError>
```

A low-level interface for reading JSON data from a byte stream

See [`Reader`] for a higher-level interface for interface with [`BufRead`]

The push-based interface facilitates integration with sources that yield arbitrarily
delimited bytes ranges, such as [`BufRead`], or a chunked byte stream received from
object storage

```
# use std::io::BufRead;
# use arrow_array::RecordBatch;
# use arrow_json::reader::{Decoder, ReaderBuilder};
# use arrow_schema::{ArrowError, SchemaRef};
#
fn read_from_json<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema).build_decoder()?;
    let mut next = move || {
        loop {
            // Decoder is agnostic that buf doesn't contain whole records
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break; // Input exhausted
            }
            let read = buf.len();
            let decoded = decoder.decode(buf)?;

            // Consume the number of bytes read
            reader.consume(decoded);
            if decoded != read {
                break; // Read batch size
            }
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

---

## DecoderContext

`struct` · `arrow_json::reader::DecoderContext`

```rust
struct DecoderContext
```

**Methods** (4)

```rust
fn coerce_primitive(&self) -> bool
fn ignore_type_conflicts(&self) -> bool
fn strict_mode(&self) -> bool
fn struct_mode(&self) -> StructMode
```

Context for decoder creation, containing configuration.

This context is passed through the decoder creation process and contains
all the configuration needed to create decoders recursively.

---

## Reader

`struct` · `arrow_json::reader::Reader`

Also reachable as `arrow::json::Reader`, `arrow_json::Reader`

```rust
struct Reader<R>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Reads JSON data with a known schema directly into arrow [`RecordBatch`]

Lines consisting solely of ASCII whitespace are ignored

---

## ReaderBuilder

`struct` · `arrow_json::reader::ReaderBuilder`

Also reachable as `arrow::json::ReaderBuilder`, `arrow_json::ReaderBuilder`

```rust
struct ReaderBuilder
```

**Methods** (9)

```rust
fn build<R: BufRead>(self, reader: R) -> Result<Reader<R>, ArrowError>
fn build_decoder(self) -> Result<Decoder, ArrowError>
fn new(schema: SchemaRef) -> Self
fn new_with_field(field: impl Into<FieldRef>) -> Self
fn with_batch_size(self, batch_size: usize) -> Self
fn with_coerce_primitive(self, coerce_primitive: bool) -> Self
fn with_ignore_type_conflicts(self, ignore_type_conflicts: bool) -> Self
fn with_strict_mode(self, strict_mode: bool) -> Self
fn with_struct_mode(self, struct_mode: StructMode) -> Self
```

A builder for [`Reader`] and [`Decoder`]

---
