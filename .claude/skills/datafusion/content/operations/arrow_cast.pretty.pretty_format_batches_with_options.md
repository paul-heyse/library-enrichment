# `arrow_cast::pretty::pretty_format_batches_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.pretty_format_batches_with_options.json).

<a id="op-85221173646a195629d41185"></a>
## pretty_format_batches_with_options

`function` · `arrow_cast::pretty::pretty_format_batches_with_options` · arrow-cast 59.3.0

```rust
fn pretty_format_batches_with_options(results: &[arrow_array::RecordBatch], options: &display::FormatOptions<'_>) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Source: `src/pretty.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a visual representation of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es with formatting options.

# Arguments
* `results` - A slice of record batches to display
* `options` - [`FormatOptions`](../operations/arrow_cast.display.FormatOptions.md#op-739ef37988037ce736e7faff) that control the resulting display

# Example
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
# use arrow_cast::display::FormatOptions;
# use arrow_cast::pretty::{pretty_format_batches, pretty_format_batches_with_options};
# let batch = RecordBatch::try_from_iter(vec![
#       ("a", Arc::new(Int32Array::from(vec![1, 2])) as ArrayRef),
#       ("b", Arc::new(StringArray::from(vec![Some("a"), None]))),
# ]).unwrap();
let options = FormatOptions::new()
  .with_null("<NULL>");
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches_with_options(&[batch], &options).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+--------+
| a | b      |
+---+--------+
| 1 | a      |
| 2 | <NULL> |
+---+--------+"#);
```
