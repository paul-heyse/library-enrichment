# `parquet::arrow`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.json).

<a id="op-b2d3d71b2a2cc696513bb543"></a>
## arrow

`module` · `parquet::arrow` · parquet 59.3.0

```rust
mod arrow
```

Source: `src/arrow/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

 API for reading/writing Arrow [`RecordBatch`]es and [`Array`]s to/from
 Parquet Files.

 See the [crate-level documentation](crate) for more details on other APIs

 # Schema Conversion

 These APIs ensure that data in Arrow [`RecordBatch`]es written to Parquet are
 read back as [`RecordBatch`]es with the exact same types and values.

 Parquet and Arrow have different type systems, and there is not
 always a one to one mapping between the systems. For example, data
 stored as a Parquet [`BYTE_ARRAY`] can be read as either an Arrow
 [`BinaryViewArray`] or [`BinaryArray`].

 To recover the original Arrow types, the writers in this module add a "hint" to
 the metadata in the [`ARROW_SCHEMA_META_KEY`](../operations/parquet.arrow.ARROW_SCHEMA_META_KEY.md#op-40f8e942a81681d7b74aaa33) key which records the original Arrow
 schema. The metadata hint follows the same convention as arrow-cpp based
 implementations such as `pyarrow`. The reader looks for the schema hint in the
 metadata to determine Arrow types, and if it is not present, infers the Arrow schema
 from the Parquet schema.

 In situations where the embedded Arrow schema is not compatible with the Parquet
 schema, the Parquet schema takes precedence and no error is raised.
 See [#1663](https://github.com/apache/arrow-rs/issues/1663)

 You can also control the type conversion process in more detail using:

 * [`ArrowSchemaConverter`] control the conversion of Arrow types to Parquet
   types.

 * [`ArrowReaderOptions::with_schema`] to explicitly specify your own Arrow schema hint
   to use when reading Parquet, overriding any metadata that may be present.

 [`RecordBatch`]: arrow_array::RecordBatch
 [`Array`]: arrow_array::Array
 [`BYTE_ARRAY`]: crate::basic::Type::BYTE_ARRAY
 [`BinaryViewArray`]: arrow_array::BinaryViewArray
 [`BinaryArray`]: arrow_array::BinaryArray
 [`ArrowReaderOptions::with_schema`]: arrow_reader::ArrowReaderOptions::with_schema

 # Example: Writing Arrow `RecordBatch` to Parquet file

```rust
 # use arrow_array::{Int32Array, ArrayRef};
 # use arrow_array::RecordBatch;
 # use parquet::arrow::arrow_writer::ArrowWriter;
 # use parquet::file::properties::WriterProperties;
 # use tempfile::tempfile;
 # use std::sync::Arc;
 # use parquet::basic::Compression;
 let ids = Int32Array::from(vec![1, 2, 3, 4]);
 let vals = Int32Array::from(vec![5, 6, 7, 8]);
 let batch = RecordBatch::try_from_iter(vec![
   ("id", Arc::new(ids) as ArrayRef),
   ("val", Arc::new(vals) as ArrayRef),
 ]).unwrap();

 let file = tempfile().unwrap();

 // WriterProperties can be used to set Parquet file options
 let props = WriterProperties::builder()
     .set_compression(Compression::SNAPPY)
     .build();

 let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props)).unwrap();

 writer.write(&batch).expect("Writing batch");

 // writer must be closed to write footer
 writer.close().unwrap();
 ```

 # Example: Reading Parquet file into Arrow `RecordBatch`

 ```rust
 # use std::fs::File;
 # use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
 # use std::sync::Arc;
 # use arrow_array::Int32Array;
 # use arrow::datatypes::{DataType, Field, Schema};
 # use arrow_array::RecordBatch;
 # use parquet::arrow::arrow_writer::ArrowWriter;
 #
 # let ids = Int32Array::from(vec![1, 2, 3, 4]);
 # let schema = Arc::new(Schema::new(vec![
 #     Field::new("id", DataType::Int32, false),
 # ]));
 #
 # let file = File::create("data.parquet").unwrap();
 #
 # let batch = RecordBatch::try_new(Arc::clone(&schema), vec![Arc::new(ids)]).unwrap();
 # let batches = vec![batch];
 #
 # let mut writer = ArrowWriter::try_new(file, Arc::clone(&schema), None).unwrap();
 #
 # for batch in batches {
 #     writer.write(&batch).expect("Writing batch");
 # }
 # writer.close().unwrap();
 #
 let file = File::open("data.parquet").unwrap();

 let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
 println!("Converted arrow schema is: {}", builder.schema());

 let mut reader = builder.build().unwrap();

 let record_batch = reader.next().unwrap().unwrap();

 println!("Read {} records.", record_batch.num_rows());
 ```

 # Example: Reading non-uniformly encrypted parquet file into arrow record batch

 Note: This requires the experimental `encryption` feature to be enabled at compile time.

```rust
 # use arrow_array::{Int32Array, ArrayRef};
 # use arrow_array::{types, RecordBatch};
 # use parquet::arrow::arrow_reader::{
 #     ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReaderBuilder,
 # };
 # use arrow_array::cast::AsArray;
 # use parquet::file::metadata::ParquetMetaData;
 # use tempfile::tempfile;
 # use std::fs::File;
 # use parquet::encryption::decrypt::FileDecryptionProperties;
 # let test_data = arrow::util::test_util::parquet_test_data();
 # let path = format!("{test_data}/encrypt_columns_and_footer.parquet.encrypted");
 #
 let file = File::open(path).unwrap();

 // Define the AES encryption keys required required for decrypting the footer metadata
 // and column-specific data. If only a footer key is used then it is assumed that the
 // file uses uniform encryption and all columns are encrypted with the footer key.
 // If any column keys are specified, other columns without a key provided are assumed
 // to be unencrypted
 let footer_key = "0123456789012345".as_bytes(); // Keys are 128 bits (16 bytes)
 let column_1_key = "1234567890123450".as_bytes();
 let column_2_key = "1234567890123451".as_bytes();

 let decryption_properties = FileDecryptionProperties::builder(footer_key.to_vec())
     .with_column_key("double_field", column_1_key.to_vec())
     .with_column_key("float_field", column_2_key.to_vec())
     .build()
     .unwrap();

 let options = ArrowReaderOptions::default()
  .with_file_decryption_properties(decryption_properties);
 let reader_metadata = ArrowReaderMetadata::load(&file, options.clone()).unwrap();
 let file_metadata = reader_metadata.metadata().file_metadata();
 assert_eq!(50, file_metadata.num_rows());

 let mut reader = ParquetRecordBatchReaderBuilder::try_new_with_options(file, options)
   .unwrap()
   .build()
   .unwrap();

 let record_batch = reader.next().unwrap().unwrap();
 assert_eq!(50, record_batch.num_rows());
 ```

Unresolved upstream links (retained, not inferred): ``ArrowSchemaConverter``.
