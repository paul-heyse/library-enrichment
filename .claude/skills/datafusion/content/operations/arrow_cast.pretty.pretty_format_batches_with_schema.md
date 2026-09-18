# `arrow_cast::pretty::pretty_format_batches_with_schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.pretty_format_batches_with_schema.json).

<a id="op-bd278151d31f0da2f1203cec"></a>
## pretty_format_batches_with_schema

`function` · `arrow_cast::pretty::pretty_format_batches_with_schema` · arrow-cast 59.3.0

```rust
fn pretty_format_batches_with_schema(schema: arrow_schema::SchemaRef, results: &[arrow_array::RecordBatch]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Source: `src/pretty.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a visual representation of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es with a provided schema.

Useful to display empty batches.

# Example
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
# use arrow_cast::pretty::pretty_format_batches_with_schema;
# use arrow_schema::{DataType, Field, Schema};
let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, true),
]));
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches_with_schema(schema, &[]).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+---+
| a | b |
+---+---+
+---+---+"#);
```
