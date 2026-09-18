# `parquet::variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.variant.json).

<a id="op-661d0febb503222365f1b4d8"></a>
## variant

`module` · `parquet::variant` · parquet 59.3.0

```rust
mod variant
```

Source: `src/variant.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

⚠️ Experimental Support for reading and writing [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d)s to / from Parquet files ⚠️

This is a 🚧 Work In Progress

Note: Requires the `variant_experimental` feature of the `parquet` crate to be enabled.

# Features
* Representation of [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d), and [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) for working with
  Variant values (see [`parquet_variant`](../modules/parquet_variant.md#op-6469913707cd0592bc49d8dc) for more details)
* Kernels for working with arrays of Variant values
  such as conversion between `Variant` and JSON, and shredding/unshredding
  (see [`parquet_variant_compute`](../modules/parquet_variant_compute.md#op-72154b4e4751c8223991ae4d) for more details)

# Example: Writing a Parquet file with Variant column
```rust
# use parquet::variant::{VariantArray, VariantType, VariantArrayBuilder, VariantBuilderExt};
# use std::sync::Arc;
# use arrow_array::{Array, ArrayRef, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::ArrowWriter;
# fn main() -> Result<(), parquet::errors::ParquetError> {
 // Use the VariantArrayBuilder to build a VariantArray
 let mut builder = VariantArrayBuilder::new(3);
 builder.new_object().with_field("name", "Alice").finish(); // row 1: {"name": "Alice"}
 builder.append_value("such wow"); // row 2: "such wow" (a string)
 let array = builder.build();

 // Since VariantArray is an ExtensionType, it needs to be converted
 // to an ArrayRef and Field with the appropriate metadata
 // before it can be written to a Parquet file
 let field = array.field("data");
 let array = ArrayRef::from(array);
 // create a RecordBatch with the VariantArray
 let schema = Schema::new(vec![field]);
 let batch = RecordBatch::try_new(Arc::new(schema), vec![array])?;

 // Now you can write the RecordBatch to the Parquet file, as normal
 let file = std::fs::File::create("variant.parquet")?;
 let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
 writer.write(&batch)?;
 writer.close()?;

# std::fs::remove_file("variant.parquet")?;
# Ok(())
# }
```

# Example: Writing JSON into a Parquet file with Variant column
```rust
# use std::sync::Arc;
# use arrow_array::{ArrayRef, RecordBatch, StringArray};
# use arrow_schema::Schema;
# use parquet::variant::{json_to_variant, VariantArray};
# use parquet::arrow::ArrowWriter;
# fn main() -> Result<(), parquet::errors::ParquetError> {
 // Create an array of JSON strings, simulating a column of JSON data
 let input_array: ArrayRef = Arc::new(StringArray::from(vec![
  Some(r#"{"name": "Alice", "age": 30}"#),
  Some(r#"{"name": "Bob", "age": 25, "address": {"city": "New York"}}"#),
  None,
  Some("{}"),
 ]));

 // Convert the JSON strings to a VariantArray
 let array: VariantArray = json_to_variant(&input_array)?;
 // create a RecordBatch with the VariantArray
 let schema = Schema::new(vec![array.field("data")]);
 let batch = RecordBatch::try_new(Arc::new(schema), vec![ArrayRef::from(array)])?;

 // write the RecordBatch to a Parquet file as normal
 let file = std::fs::File::create("variant-json.parquet")?;
 let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
 writer.write(&batch)?;
 writer.close()?;
# std::fs::remove_file("variant-json.parquet")?;
# Ok(())
# }
```

# Example: Reading a Parquet file with Variant column

Use the [`VariantType`](../operations/parquet_variant_compute.variant_array.VariantType.md#op-88d7342856958a338cda9720) extension type to find the Variant column:

```
# use std::sync::Arc;
# use std::path::PathBuf;
# use arrow_array::{ArrayRef, RecordBatch, RecordBatchReader};
# use parquet::variant::{Variant, VariantArray, VariantType};
# use parquet::arrow::arrow_reader::ArrowReaderBuilder;
# fn main() -> Result<(), parquet::errors::ParquetError> {
# use arrow_array::StructArray;
# fn file_path() -> PathBuf { // return a testing file path
#    PathBuf::from(arrow::util::test_util::parquet_test_data())
#   .join("..")
#   .join("shredded_variant")
#   .join("case-075.parquet")
# }
// Read the Parquet file using standard Arrow Parquet reader.
// Note this file has 2 columns: "id", "var", and the "var" column
let file = std::fs::File::open(file_path())?;
let mut reader = ArrowReaderBuilder::try_new(file)?.build()?;

// You can check if a column contains a Variant using
// the VariantType extension type
let schema = reader.schema();
let field = schema.field_with_name("var")?;
assert!(field.has_valid_extension_type::<VariantType>());

// The reader will yield RecordBatches with a StructArray
// to convert them to VariantArray, use VariantArray::try_new
let batch = reader.next().unwrap().unwrap();

let col = batch.column_by_name("var").unwrap();
let var_array = VariantArray::try_new(col)?;
assert_eq!(var_array.len(), 1);
let var_value: Variant = var_array.value(0);
assert_eq!(var_value, Variant::from("iceberg")); // the value in case-075.parquet
# Ok(())
# }
```
