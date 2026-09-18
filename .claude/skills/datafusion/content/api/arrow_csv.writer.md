# `arrow_csv::writer`

Crate `arrow-csv` · 3 public items · structured records in [`model/arrow_csv.writer.json`](../model/arrow_csv.writer.json)

## Terminator

`enum` · `arrow_csv::writer::Terminator`

```rust
enum Terminator
```

**Variants**: `CRLF`, `Any`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/arrow_csv.writer.Terminator.md).


The line terminator to use when writing CSV files.

---

## Writer

`struct` · `arrow_csv::writer::Writer`

Also reachable as `arrow::csv::Writer`, `arrow_csv::Writer`

```rust
struct Writer<W: Write>
```

**Implements**: `arrow_array::record_batch::RecordBatchWriter`

**Derives**: Debug

**Methods** (3)

```rust
fn into_inner(self) -> W
fn new(writer: W) -> Self
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

**via `arrow_array::record_batch::RecordBatchWriter`**

```rust
fn close(self) -> Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_csv.writer.Writer.md).


A CSV writer

See the [module documentation](crate::writer) for examples.

---

## WriterBuilder

`struct` · `arrow_csv::writer::WriterBuilder`

Also reachable as `arrow::csv::WriterBuilder`, `arrow_csv::WriterBuilder`

```rust
struct WriterBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (32)

```rust
fn build<W: Write>(self, writer: W) -> Writer<W>
fn date_format(&self) -> Option<&str>
fn datetime_format(&self) -> Option<&str>
fn delimiter(&self) -> u8
fn double_quote(&self) -> bool
fn escape(&self) -> u8
fn header(&self) -> bool
fn ignore_leading_whitespace(&self) -> bool
fn ignore_trailing_whitespace(&self) -> bool
fn line_terminator(&self) -> &Terminator
fn new() -> Self
fn null(&self) -> &str
fn quote(&self) -> u8
fn quote_style(&self) -> QuoteStyle
fn time_format(&self) -> Option<&str>
fn timestamp_format(&self) -> Option<&str>
fn timestamp_tz_format(&self) -> Option<&str>
fn with_date_format(self, format: String) -> Self
fn with_datetime_format(self, format: String) -> Self
fn with_delimiter(self, delimiter: u8) -> Self
fn with_double_quote(self, double_quote: bool) -> Self
fn with_escape(self, escape: u8) -> Self
fn with_header(self, header: bool) -> Self
fn with_ignore_leading_whitespace(self, ignore: bool) -> Self
fn with_ignore_trailing_whitespace(self, ignore: bool) -> Self
fn with_line_terminator(self, terminator: Terminator) -> Self
fn with_null(self, null_value: String) -> Self
fn with_quote(self, quote: u8) -> Self
fn with_quote_style(self, quote_style: QuoteStyle) -> Self
fn with_time_format(self, format: String) -> Self
fn with_timestamp_format(self, format: String) -> Self
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_csv.writer.WriterBuilder.md).


A CSV writer builder

---
