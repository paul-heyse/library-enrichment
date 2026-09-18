# `arrow_avro::writer::AvroWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.AvroWriter.json).

<a id="op-109f7d6c1f115d2ec6574736"></a>
## AvroWriter

`type_alias` · `arrow_avro::writer::AvroWriter` · arrow-avro 59.3.0

```rust
type AvroWriter<W> = Writer<W, writer::format::AvroOcfFormat>
```

Source: `src/writer/mod.rs:613`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Alias for an Avro **Object Container File** writer.

### Quickstart (runnable)

```
use std::io::Cursor;
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, StringArray, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroWriter;
use arrow_avro::reader::ReaderBuilder;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Writer schema: { id: long, name: string }
let writer_schema = Schema::new(vec![
    Field::new("id", DataType::Int64, false),
    Field::new("name", DataType::Utf8, false),
]);

// Build a RecordBatch with two rows
let batch = RecordBatch::try_new(
    Arc::new(writer_schema.clone()),
    vec![
        Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
        Arc::new(StringArray::from(vec!["a", "b"])) as ArrayRef,
    ],
)?;

// Write an Avro **Object Container File** (OCF) to memory
let mut w = AvroWriter::new(Vec::<u8>::new(), writer_schema.clone())?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();

// Build a Reader and decode the batch back
let mut r = ReaderBuilder::new().build(Cursor::new(bytes))?;
let out = r.next().unwrap()?;
assert_eq!(out.num_rows(), 2);
# Ok(()) }
```
