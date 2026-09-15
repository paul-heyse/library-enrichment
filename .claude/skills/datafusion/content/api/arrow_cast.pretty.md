# `arrow_cast::pretty`

Crate `arrow-cast` · 7 public items · structured records in [`model/arrow_cast.pretty.json`](../model/arrow_cast.pretty.json)

## pretty_format_batches

`function` · `arrow_cast::pretty::pretty_format_batches`

Also reachable as `arrow::util::pretty::pretty_format_batches`

```rust
fn pretty_format_batches(results: &[arrow_array::RecordBatch]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Create a visual representation of [`RecordBatch`]es

Uses default values for display. See [`pretty_format_batches_with_options`]
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

---

## pretty_format_batches_with_options

`function` · `arrow_cast::pretty::pretty_format_batches_with_options`

Also reachable as `arrow::util::pretty::pretty_format_batches_with_options`

```rust
fn pretty_format_batches_with_options(results: &[arrow_array::RecordBatch], options: &display::FormatOptions<'_>) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Create a visual representation of [`RecordBatch`]es with formatting options.

# Arguments
* `results` - A slice of record batches to display
* `options` - [`FormatOptions`] that control the resulting display

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

---

## pretty_format_batches_with_schema

`function` · `arrow_cast::pretty::pretty_format_batches_with_schema`

Also reachable as `arrow::util::pretty::pretty_format_batches_with_schema`

```rust
fn pretty_format_batches_with_schema(schema: arrow_schema::SchemaRef, results: &[arrow_array::RecordBatch]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Create a visual representation of [`RecordBatch`]es with a provided schema.

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

---

## pretty_format_columns

`function` · `arrow_cast::pretty::pretty_format_columns`

Also reachable as `arrow::util::pretty::pretty_format_columns`

```rust
fn pretty_format_columns(col_name: &str, results: &[arrow_array::ArrayRef]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Create a visual representation of [`ArrayRef`]

Uses default values for display. See [`pretty_format_columns_with_options`]

See [`pretty_format_batches`] for an example

---

## pretty_format_columns_with_options

`function` · `arrow_cast::pretty::pretty_format_columns_with_options`

Also reachable as `arrow::util::pretty::pretty_format_columns_with_options`

```rust
fn pretty_format_columns_with_options(col_name: &str, results: &[arrow_array::ArrayRef], options: &display::FormatOptions<'_>) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Create a visual representation of [`ArrayRef`] with formatting options.

See [`pretty_format_batches_with_options`] for an example

---

## print_batches

`function` · `arrow_cast::pretty::print_batches`

Also reachable as `arrow::util::pretty::print_batches`

```rust
fn print_batches(results: &[arrow_array::RecordBatch]) -> Result<(), arrow_schema::ArrowError>
```

Prints a visual representation of record batches to stdout

---

## print_columns

`function` · `arrow_cast::pretty::print_columns`

Also reachable as `arrow::util::pretty::print_columns`

```rust
fn print_columns(col_name: &str, results: &[arrow_array::ArrayRef]) -> Result<(), arrow_schema::ArrowError>
```

Prints a visual representation of a list of column to stdout

---
