# `arrow_json::writer::ArrayWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.ArrayWriter.json).

<a id="op-63ae7f4f9382b724e93f9b5b"></a>
## ArrayWriter

`type_alias` · `arrow_json::writer::ArrayWriter` · arrow-json 59.3.0

```rust
type ArrayWriter<W> = Writer<W, JsonArray>
```

Source: `src/writer/mod.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A JSON writer which serializes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to JSON arrays.
