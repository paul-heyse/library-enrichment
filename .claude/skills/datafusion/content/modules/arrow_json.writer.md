# `arrow_json::writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.json).

<a id="op-3211eef7ff6c9c882d353606"></a>
## writer

`module` · `arrow_json::writer` · arrow-json 59.3.0

```rust
mod writer
```

Source: `src/writer/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

 # JSON Writer

 This JSON writer converts Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es into arrays of
 JSON objects or JSON formatted byte streams.

 ## Writing JSON formatted byte streams

 To serialize [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es into line-delimited JSON bytes, use
 [`LineDelimitedWriter`](../operations/arrow_json.writer.LineDelimitedWriter.md#op-7591469afc32a579cb43de2f):

 ```
 # use std::sync::Arc;
 # use arrow_array::{Int32Array, RecordBatch};
 # use arrow_schema::{DataType, Field, Schema};

 let schema = Schema::new(vec![Field::new("a", DataType::Int32, false)]);
 let a = Int32Array::from(vec![1, 2, 3]);
 let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a)]).unwrap();

 // Write the record batch out as JSON
 let buf = Vec::new();
 let mut writer = arrow_json::LineDelimitedWriter::new(buf);
 writer.write_batches(&vec![&batch]).unwrap();
 writer.finish().unwrap();

 // Get the underlying buffer back,
 let buf = writer.into_inner();
 assert_eq!(r#"{"a":1}
 {"a":2}
 {"a":3}
"#, String::from_utf8(buf).unwrap())
 ```

 To serialize [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es into a well formed JSON array, use
 [`ArrayWriter`](../operations/arrow_json.writer.ArrayWriter.md#op-63ae7f4f9382b724e93f9b5b):

 ```
 # use std::sync::Arc;
 # use arrow_array::{Int32Array, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};

 let schema = Schema::new(vec![Field::new("a", DataType::Int32, false)]);
 let a = Int32Array::from(vec![1, 2, 3]);
 let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a)]).unwrap();

 // Write the record batch out as a JSON array
 let buf = Vec::new();
 let mut writer = arrow_json::ArrayWriter::new(buf);
 writer.write_batches(&vec![&batch]).unwrap();
 writer.finish().unwrap();

 // Get the underlying buffer back,
 let buf = writer.into_inner();
 assert_eq!(r#"[{"a":1},{"a":2},{"a":3}]"#, String::from_utf8(buf).unwrap())
 ```

 [`LineDelimitedWriter`](../operations/arrow_json.writer.LineDelimitedWriter.md#op-7591469afc32a579cb43de2f) and [`ArrayWriter`](../operations/arrow_json.writer.ArrayWriter.md#op-63ae7f4f9382b724e93f9b5b) will omit writing keys with null values.
 In order to explicitly write null values for keys, configure a custom [`Writer`](../operations/arrow_json.writer.Writer.md#op-e929064a2c0f20a280aeb9ef) by
 using a [`WriterBuilder`](../operations/arrow_json.writer.WriterBuilder.md#op-eab011f6b5e700c1657bc7e0) to construct a [`Writer`](../operations/arrow_json.writer.Writer.md#op-e929064a2c0f20a280aeb9ef).

 ## Writing to [serde_json] JSON Objects

 To serialize [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es into an array of
 [JSON](https://docs.serde.rs/serde_json/) objects you can reparse the resulting JSON string.
 Note that this is less efficient than using the `Writer` API.

 ```
 # use std::sync::Arc;
 # use arrow_array::{Int32Array, RecordBatch};
 # use arrow_schema::{DataType, Field, Schema};
 let schema = Schema::new(vec![Field::new("a", DataType::Int32, false)]);
 let a = Int32Array::from(vec![1, 2, 3]);
 let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a)]).unwrap();

 // Write the record batch out as json bytes (string)
 let buf = Vec::new();
 let mut writer = arrow_json::ArrayWriter::new(buf);
 writer.write_batches(&vec![&batch]).unwrap();
 writer.finish().unwrap();
 let json_data = writer.into_inner();

 // Parse the string using serde_json
 use serde_json::{Map, Value};
 let json_rows: Vec<Map<String, Value>> = serde_json::from_reader(json_data.as_slice()).unwrap();
 assert_eq!(
     serde_json::Value::Object(json_rows[1].clone()),
     serde_json::json!({"a": 2}),
 );
 ```

Unresolved upstream links (retained, not inferred): `serde_json`.
