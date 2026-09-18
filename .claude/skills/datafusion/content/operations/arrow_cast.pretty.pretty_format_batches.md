# `arrow_cast::pretty::pretty_format_batches`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.pretty_format_batches.json).

<a id="op-42add46948080a47d285119f"></a>
## pretty_format_batches

`function` · `arrow_cast::pretty::pretty_format_batches` · arrow-cast 59.3.0

```rust
fn pretty_format_batches(results: &[arrow_array::RecordBatch]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Source: `src/pretty.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a visual representation of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es

Uses default values for display. See [`pretty_format_batches_with_options`](../operations/arrow_cast.pretty.pretty_format_batches_with_options.md#op-85221173646a195629d41185)
for more control.

# Example
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
# use arrow_cast::pretty::pretty_format_batches;
# let batch = RecordBatch::try_from_iter(vec![
#       ("a", Arc::new(Int32Array::from(vec![1, 2, 3, 4, 5])) as ArrayRef),
#       ("b", Arc::new(StringArray::from(vec![Some("a"), Some("b"), None, Some("d"), Some("e")]))),
# ]).unwrap();
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches(&[batch]).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+---+
| a | b |
+---+---+
| 1 | a |
| 2 | b |
| 3 |   |
| 4 | d |
| 5 | e |
+---+---+"#);
```
