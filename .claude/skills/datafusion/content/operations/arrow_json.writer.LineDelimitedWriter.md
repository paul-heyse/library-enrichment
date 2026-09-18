# `arrow_json::writer::LineDelimitedWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.LineDelimitedWriter.json).

<a id="op-7591469afc32a579cb43de2f"></a>
## LineDelimitedWriter

`type_alias` · `arrow_json::writer::LineDelimitedWriter` · arrow-json 59.3.0

```rust
type LineDelimitedWriter<W> = Writer<W, LineDelimited>
```

Source: `src/writer/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A JSON writer which serializes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to newline delimited JSON objects.
