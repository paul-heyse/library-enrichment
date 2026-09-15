# `arrow_csv::reader`

Crate `arrow-csv` · 6 public items · structured records in [`model/arrow_csv.reader.json`](../model/arrow_csv.reader.json)

## infer_schema_from_files

`function` · `arrow_csv::reader::infer_schema_from_files`

Also reachable as `arrow::csv::infer_schema_from_files`, `arrow_csv::infer_schema_from_files`

```rust
fn infer_schema_from_files(files: &[String], delimiter: u8, max_read_records: Option<usize>, has_header: bool) -> Result<Schema, ArrowError>
```

Infer schema from a list of CSV files by reading through first n records
with `max_read_records` controlling the maximum number of records to read.

Files will be read in the given order until n records have been reached.

If `max_read_records` is not set, all files will be read fully to infer the schema.

---

## BufReader

`struct` · `arrow_csv::reader::BufReader`

```rust
struct BufReader<R>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn schema(&self) -> SchemaRef
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

CSV file reader implementation. See [`Reader`] for usage

Despite having the same name as [`std::io::BufReader`, this structure does
not buffer reads itself

---

## Decoder

`struct` · `arrow_csv::reader::Decoder`

```rust
struct Decoder
```

**Derives**: Debug

**Methods** (3)

```rust
fn capacity(&self) -> usize
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

A push-based interface for decoding CSV data from an arbitrary byte stream

See [`Reader`] for a higher-level interface for interface with [`Read`]

The push-based interface facilitates integration with sources that yield arbitrarily
delimited bytes ranges, such as [`BufRead`], or a chunked byte stream received from
object storage

```
# use std::io::BufRead;
# use arrow_array::RecordBatch;
# use arrow_csv::ReaderBuilder;
# use arrow_schema::{ArrowError, SchemaRef};
#
fn read_from_csv<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
    batch_size: usize,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema)
        .with_batch_size(batch_size)
        .build_decoder();

    let mut next = move || {
        loop {
            let buf = reader.fill_buf()?;
            let decoded = decoder.decode(buf)?;
            if decoded == 0 {
                break;
            }

            // Consume the number of bytes read
            reader.consume(decoded);
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

---

## Format

`struct` · `arrow_csv::reader::Format`

```rust
struct Format
```

**Derives**: Clone, Debug, Default

**Methods** (10)

```rust
fn infer_schema<R: Read>(&self, reader: R, max_records: Option<usize>) -> Result<(Schema, usize), ArrowError>
fn with_comment(self, comment: u8) -> Self
fn with_delimiter(self, delimiter: u8) -> Self
fn with_escape(self, escape: u8) -> Self
fn with_header(self, has_header: bool) -> Self
fn with_header_validation(self, validate_header: bool) -> Self
fn with_null_regex(self, null_regex: Regex) -> Self
fn with_quote(self, quote: u8) -> Self
fn with_terminator(self, terminator: u8) -> Self
fn with_truncated_rows(self, allow: bool) -> Self
```

The format specification for the CSV file

---

## ReaderBuilder

`struct` · `arrow_csv::reader::ReaderBuilder`

Also reachable as `arrow::csv::ReaderBuilder`, `arrow_csv::ReaderBuilder`

```rust
struct ReaderBuilder
```

**Derives**: Debug

**Methods** (17)

```rust
fn build<R: Read>(self, reader: R) -> Result<Reader<R>, ArrowError>
fn build_buffered<R: BufRead>(self, reader: R) -> Result<BufReader<R>, ArrowError>
fn build_decoder(self) -> Decoder
fn new(schema: SchemaRef) -> ReaderBuilder
fn with_batch_size(self, batch_size: usize) -> Self
fn with_bounds(self, start: usize, end: usize) -> Self
fn with_comment(self, comment: u8) -> Self
fn with_delimiter(self, delimiter: u8) -> Self
fn with_escape(self, escape: u8) -> Self
fn with_format(self, format: Format) -> Self
fn with_header(self, has_header: bool) -> Self
fn with_header_validation(self, validate_header: bool) -> Self
fn with_null_regex(self, null_regex: Regex) -> Self
fn with_projection(self, projection: Vec<usize>) -> Self
fn with_quote(self, quote: u8) -> Self
fn with_terminator(self, terminator: u8) -> Self
fn with_truncated_rows(self, allow: bool) -> Self
```

Builder for CSV [`Reader`]s

---

## Reader

`type_alias` · `arrow_csv::reader::Reader`

Also reachable as `arrow::csv::Reader`, `arrow_csv::Reader`

```rust
type Reader<R> = BufReader<std::io::BufReader<R>>
```

CSV file reader using [`std::io::BufReader`]

See [`ReaderBuilder`] to construct a CSV reader with options and  the
[module-level documentation](crate::reader) for more details and examples

---
