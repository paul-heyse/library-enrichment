# `arrow_avro::writer::AvroStreamWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.AvroStreamWriter.json).

<a id="op-74718ef152874c77b8b4f550"></a>
## AvroStreamWriter

`type_alias` · `arrow_avro::writer::AvroStreamWriter` · arrow-avro 59.3.0

```rust
type AvroStreamWriter<W> = Writer<W, writer::format::AvroSoeFormat>
```

Source: `src/writer/mod.rs:646`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Alias for an Avro **Single Object Encoding** stream writer.

### Example

This writer automatically adds the appropriate per-record prefix (based on the
fingerprint strategy) before the Avro body of each record. The default is Single
Object Encoding (SOE) with a Rabin fingerprint.

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroStreamWriter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// One‑column Arrow batch
let schema = Schema::new(vec![Field::new("x", DataType::Int64, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int64Array::from(vec![10, 20])) as ArrayRef],
)?;

// Write an Avro Single Object Encoding stream to a Vec<u8>
let sink: Vec<u8> = Vec::new();
let mut w = AvroStreamWriter::new(sink, schema)?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();
assert!(!bytes.is_empty());
# Ok(()) }
```
