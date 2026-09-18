# `arrow_csv::writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.writer.json).

<a id="op-57648637b865f7acdab34be8"></a>
## writer

`module` · `arrow_csv::writer` · arrow-csv 59.3.0

```rust
mod writer
```

Source: `src/writer.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

CSV Writing: [`Writer`](../operations/arrow_csv.writer.Writer.md#op-9112f918255e66827bb62542) and [`WriterBuilder`](../operations/arrow_csv.writer.WriterBuilder.md#op-44e2845dc1cd29d45ec5dba4)

This CSV writer allows Arrow data (in record batches) to be written as CSV files.
The writer does not support writing `ListArray` and `StructArray`.

# Example
```
# use arrow_array::*;
# use arrow_array::types::*;
# use arrow_csv::Writer;
# use arrow_schema::*;
# use std::sync::Arc;

let schema = Schema::new(vec![
    Field::new("c1", DataType::Utf8, false),
    Field::new("c2", DataType::Float64, true),
    Field::new("c3", DataType::UInt32, false),
    Field::new("c4", DataType::Boolean, true),
]);
let c1 = StringArray::from(vec![
    "Lorem ipsum dolor sit amet",
    "consectetur adipiscing elit",
    "sed do eiusmod tempor",
]);
let c2 = PrimitiveArray::<Float64Type>::from(vec![
    Some(123.564532),
    None,
    Some(-556132.25),
]);
let c3 = PrimitiveArray::<UInt32Type>::from(vec![3, 2, 1]);
let c4 = BooleanArray::from(vec![Some(true), Some(false), None]);

let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![Arc::new(c1), Arc::new(c2), Arc::new(c3), Arc::new(c4)],
)
.unwrap();

let mut output = Vec::with_capacity(1024);

let mut writer = Writer::new(&mut output);
let batches = vec![&batch, &batch];
for batch in batches {
    writer.write(batch).unwrap();
}
```

# Whitespace Handling

The writer supports trimming leading and trailing whitespace from string values,
compatible with Apache Spark's CSV options `ignoreLeadingWhiteSpace` and
`ignoreTrailingWhiteSpace`. This is useful when working with data that may have
unwanted padding.

Whitespace trimming is applied to all string data types:
- `DataType::Utf8`
- `DataType::LargeUtf8`
- `DataType::Utf8View`

## Example: Use [`WriterBuilder`](../operations/arrow_csv.writer.WriterBuilder.md#op-44e2845dc1cd29d45ec5dba4) to control whitespace handling

```
# use arrow_array::*;
# use arrow_csv::WriterBuilder;
# use arrow_schema::*;
# use std::sync::Arc;
let schema = Schema::new(vec![
    Field::new("name", DataType::Utf8, false),
    Field::new("comment", DataType::Utf8, false),
]);

let name = StringArray::from(vec![
    "  Alice  ",   // Leading and trailing spaces
    "Bob",         // No spaces
    "  Charlie",   // Leading spaces only
]);
let comment = StringArray::from(vec![
    "  Great job!  ",
    "Well done",
    "Excellent  ",
]);

let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![Arc::new(name), Arc::new(comment)],
)
.unwrap();

// Trim both leading and trailing whitespace
let mut output = Vec::new();
WriterBuilder::new()
    .with_ignore_leading_whitespace(true)
    .with_ignore_trailing_whitespace(true)
    .build(&mut output)
    .write(&batch)
    .unwrap();
assert_eq!(
    String::from_utf8(output).unwrap(),
    "\
name,comment\n\
Alice,Great job!\n\
Bob,Well done\n\
Charlie,Excellent\n"
);
```

# Quoting Styles

The writer supports different quoting styles for fields, compatible with Apache Spark's
CSV options like `quoteAll`. You can control when fields are quoted using the
[`QuoteStyle`] enum.

## Example

```
# use arrow_array::*;
# use arrow_csv::{WriterBuilder, QuoteStyle};
# use arrow_schema::*;
# use std::sync::Arc;

let schema = Schema::new(vec![
    Field::new("product", DataType::Utf8, false),
    Field::new("price", DataType::Float64, false),
]);

let product = StringArray::from(vec!["apple", "banana,organic", "cherry"]);
let price = Float64Array::from(vec![1.50, 2.25, 3.00]);

let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![Arc::new(product), Arc::new(price)],
)
.unwrap();

// Default behavior (QuoteStyle::Necessary)
let mut output = Vec::new();
WriterBuilder::new()
    .build(&mut output)
    .write(&batch)
    .unwrap();
assert_eq!(
    String::from_utf8(output).unwrap(),
    "product,price\napple,1.5\n\"banana,organic\",2.25\ncherry,3.0\n"
);

// Quote all fields (Spark's quoteAll=true)
let mut output = Vec::new();
WriterBuilder::new()
    .with_quote_style(QuoteStyle::Always)
    .build(&mut output)
    .write(&batch)
    .unwrap();
assert_eq!(
    String::from_utf8(output).unwrap(),
    "\"product\",\"price\"\n\"apple\",\"1.5\"\n\"banana,organic\",\"2.25\"\n\"cherry\",\"3.0\"\n"
);
```

Unresolved upstream links (retained, not inferred): ``QuoteStyle``.
